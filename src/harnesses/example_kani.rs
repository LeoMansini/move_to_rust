fn sum_of_first_and_last(slice: &[u8]) -> u8 {
    // PRECONDICIÓN: El 'slice' no debe estar vacío.
    // Si slice.len() == 0, la siguiente línea causará un panic.
    let first = slice[0];
    let last = slice[slice.len() - 1];

    // Usamos saturating_add para evitar desbordamientos y enfocarnos en la precondición.
    first.saturating_add(last)
}


#[kani::proof]
fn check_sum_harness() {
    // 1. Entrada no determinística
    let v: Vec<u8> = kani::vec::any_vec::<u8, 5>();

    // 2. Establecimiento de la precondición
    //kani::assume(!v.is_empty());

    // 3. Llamada a la función bajo la precondición
    sum_of_first_and_last(&v);
}