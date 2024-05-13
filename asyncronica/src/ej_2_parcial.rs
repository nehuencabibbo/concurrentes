use std::task;

//Leer archivo 100 lineas, c/linea sitio (URL)

fn leer_archivos() {
    pass
}

async fn request_and_time(url: String) -> int {
    let start_time = std::time::now();
    //Sleep random
    task::sleep(random(1..100)).await

    let finish_time = std::time::now()
}

async fn async_main() {
    //Leer los 100 archivos
    archivos: Vec<String> = leer_archivos().map(|nombre| request_and_time(nombre))

    //Para cada request, hacer el request y timearlo
    res = 0;
    for i in range(0, 100, n):
        let results = join_all(archivos[i, i + n]).await

        res += results.reduce(|resultado_actual, valor_vector| resultado_actual += valor_vector)

    res / 100
}

fn main() {
    task::block_on(async_main())
}