import re
import argparse
from pathlib import Path

def move_to_rust(move_code):
    # Define regex patterns and replacements
    syntax_replacements = [
        (r'module\s+([a-zA-Z0-9_]+)::([a-zA-Z0-9_]+)', r'impl \1__\2'), # "Module address::name"
        (r'resource\s+struct', r'struct'), # Structs
        (r'public', r'pub'), # Public to pub
        (r'struct\s+([a-zA-Z0-9_]+)\s+has\s+.*?\s*{', r'struct \1 {'), # Remove Struct traits
        (r'fun', r'fn'), # Fun to fn
        (r':\s*([a-zA-Z0-9_]+)\s*{', r' -> \1 {'), # Return type from ":" to "->"
        # Add more replacements here as needed
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
        remove_test_functions
    ]
    
    rust_code = move_code
    # Apply replacements
    for pattern, replacement in regex_replacements:
        rust_code = re.sub(pattern, replacement, rust_code)
    
    for replace_func in func_replacements:
        rust_code = replace_func(rust_code)
    
    return rust_code

def process_files(input_filepath):
    try:
        # Read Move code from file
        with open(input_filepath, 'r') as move_file:
            move_code = move_file.read()
        
        # Convert to Rust
        rust_code = move_to_rust(move_code)
        
        # Write Rust code to output file
        output_filepath = input_filepath.replace(".move", ".rs").replace("move/", "rust/")
        with open(output_filepath, 'w') as rust_file:
            rust_file.write(rust_code)
        
        print(f"Translation complete! Rust code written to {output_filepath}")
    
    except FileNotFoundError:
        print(f"Error: The file {input_filepath} was not found.")
    except Exception as e:
        print(f"An error occurred: {e}")


def remove_test_functions(code):
    lines = code.splitlines()
    result_lines = []
    
    i = 0
    while i < len(lines):
        if '#[test]' in lines[i]: # After a test
            while "{" not in lines[i] and i < len(lines): # Eventually the function definition starts
                i+=1
            
            i+=1
            braces_count = 1

            while braces_count > 0 and i < len(lines): # While inside function, advance i
                # Handle counting braces
                braces_count += lines[i].count('{')
                braces_count -= lines[i].count('}')
                i += 1
        
        # Append lines that are not inside the skip region
        else:
            result_lines.append(lines[i])
            i += 1

    return '\n'.join(result_lines)

if __name__ == "__main__":
    # Set up argument parser
    parser = argparse.ArgumentParser(description="Translate Move code to Rust.")
    parser.add_argument("input", help="Path to the input Move file.")
    
    # Parse arguments
    args = parser.parse_args()

    # Process the files with the provided paths
    process_files(args.input)
