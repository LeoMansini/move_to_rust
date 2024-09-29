import re
import argparse
from pathlib import Path

def move_to_rust(move_code):
    # Define regex patterns and replacements
    syntax_replacements = [
        (r'module\s+([a-zA-Z0-9_]+)::([a-zA-Z0-9_]+)', r'pub struct \1__\2 {}\nimpl \1__\2'), # "Module address::name"
        (r'resource\s+struct', r'struct'), # Structs
        (r'public', r'pub'), # Public to pub
        (r'struct\s+([a-zA-Z0-9_]+<.*?>?)\s+has\s+.*?\s*{', r'struct \1 {'), # Remove Struct traits
        (r'\(package\)', r''), # Remove 'package' scope
        (r'entry fun', r'fun'), # Remove 'Entry'
        (r'fun', r'fn'), # Fun to fn
        (r':\s*([a-zA-Z0-9_]+)\s*{', r' -> \1 {'), # Return type from ":" to "->"
        (r'option::is_some\(&(\w+\.\w+)\)', r'\1.is_some()'), # Option is_some
        (r'option::is_none\(&(\w+\.\w+)\)', r'\1.is_none()'), # Option is_none
        (r'option::fill\(&mut (\w+\.\w+), (\w+)\)', r'\1.replace(\2).unwrap()'), # Option fill (assignment if is None)
        (r'option::extract\(&mut (\w+\.\w+)\)', r'\1.take().unwrap()'), # Option extract (take)
        (r'option::none\(\)', r'None'), # Option None
        (r'assert!\((.+?),\s*(.+?)\)', r'assert!(\1, "{}", \2)'), # Assert with string literal
        (r'ctx: &mut TxContext', r''), # Remove TxContxt. TODO: Might need to model this.
        (r'phantom ', r''), # Remove phantom
    ]

    simplification_replacements = [
        (r'UID', r'u8'), # Assume UID is u8
        (r'// === Tests ===', ''), # Common comment, tests are removed so this is also
        (r'#\[test_only\].*\n', ''),  # Remove test only imports
    ]

    regex_replacements = [
        *syntax_replacements,
        *simplification_replacements
    ]

    func_replacements = [
        remove_test_functions,
        add_new_object_mock,
        move_structs_and_consts_to_global_scope,
        remove_duplicate_line_breaks,
    ]
    
    rust_code = move_code
    # Apply replacements
    for pattern, replacement in regex_replacements:
        rust_code = re.sub(pattern, replacement, rust_code)
    
    for replace_func in func_replacements:
        rust_code = replace_func(rust_code)
    
    return rust_code

def remove_test_functions(code):
    lines = code.splitlines()
    result_lines = []
    
    i = 0
    while i < len(lines):
        if '#[test]' in lines[i]: # After a test
            i = _get_end_of_scope_line(lines, i)
        
        # Append lines that are not inside the skip region
        else:
            result_lines.append(lines[i])
            i += 1

    return '\n'.join(result_lines)

def move_structs_and_consts_to_global_scope(code):
    lines = code.splitlines()
    result_lines = []

    i = 0
    braces_count = 0

    while i < len(lines):
        braces_count += lines[i].count('{')
        braces_count -= lines[i].count('}')
        if braces_count > 1:
            if (
                (lines[i].strip().startswith("struct"))
                or (lines[i].strip().startswith("pub struct"))
            ):
                end_of_inside_struct = _get_end_of_scope_line(lines, i)
                struct_lines = lines[i:end_of_inside_struct + 1]
                struct_lines = _remove_indentation_from_lines(struct_lines)
                result_lines = struct_lines + result_lines
                i = end_of_inside_struct + 1
                continue
            elif lines[i].strip().startswith("const"):
                const_line = [lines[i]]
                const_line = _remove_indentation_from_lines(const_line)
                result_lines = const_line + ["\n"] + result_lines
                i += 1
                continue

        result_lines.append(lines[i])
        i += 1

    return '\n'.join(result_lines)

def _remove_indentation_from_lines(lines):
    indentation = len(lines[0]) - len(lines[0].lstrip())
    return [l[indentation:] for l in lines]

def _get_end_of_scope_line(lines, start_line):
    i = start_line
    while "{" not in lines[i] and i < len(lines): # Eventually the function definition starts
        i+=1
    
    i+=1
    braces_count = 1

    while braces_count > 0 and i < len(lines): # While inside function, advance i
        # Handle counting braces
        braces_count += lines[i].count('{')
        braces_count -= lines[i].count('}')
        i += 1

    return i


def add_new_object_mock(code):
    """Replaces calls to object::new(ctx) which assigns a specific UID in the blockchain
    with calls to a local counter."""
    id_getter_code = '''
use std::sync::LazyLock;

pub struct IdGetter {
    current_id: std::sync::Mutex<u8>,
}

impl IdGetter {
    pub fn new() -> Self {
        IdGetter {
            current_id: std::sync::Mutex::new(0),
        }
    }

    pub fn get_new_id(&self) -> u8 {
        let mut id = self.current_id.lock().unwrap();
        *id += 1;
        *id
    }
}

// Use LazyLock to initialize ID_GETTER
pub static ID_GETTER: LazyLock<IdGetter> = LazyLock::new(|| IdGetter::new());

'''
    code = id_getter_code + code
    code = re.sub(r'object::new\(\w*\)', 'ID_GETTER.get_new_id()', code, flags=re.MULTILINE)
    return code

def remove_duplicate_line_breaks(code):
    lines = code.splitlines()
    i = 1
    while i < len(lines):
        if len(lines[i].strip()) == 0 and len(lines[i-1].strip()) == 0:
            del lines[i]
        else:
            i +=1
    return "\n".join(lines)

def process_files(input_filepath):
    try:
        # Read Move code from file
        with open(input_filepath, 'r') as move_file:
            move_code = move_file.read()
        
        # Convert to Rust
        rust_code = move_to_rust(move_code)
        
        # Write Rust code to output file
        output_filepath = input_filepath.replace(".move", ".rs")
        with open(output_filepath, 'w') as rust_file:
            rust_file.write(rust_code)
        
        print(f"Translation complete! Rust code written to {output_filepath}")
    
    except FileNotFoundError:
        print(f"Error: The file {input_filepath} was not found.")
    except Exception as e:
        print(f"An error occurred: {e}")



if __name__ == "__main__":
    # Set up argument parser
    parser = argparse.ArgumentParser(description="Translate Move code to Rust.")
    parser.add_argument("input", help="Path to the input Move file.")
    
    # Parse arguments
    args = parser.parse_args()

    # Process the files with the provided paths
    process_files(args.input)
