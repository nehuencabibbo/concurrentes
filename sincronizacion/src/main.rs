mod barbero;

use barbero::barbero;

use std::sync::Arc;

use std::sync::Mutex;

use std::sync::Convar;

fn pseudo_lector_escritor() {
    //alguienLeyendo alguienEscribiendo
    let variables = Arc::new(Mutex::new((counter, false)));
    let convar = Arc::new(Convar::new());

    //Lee
    std::thread::spawn(move || {
        {
            let a = convar.wait_while(variables.lock().unwrap(), |lock| {!lock[1]})
            a += 1
        }

        //Lees
        
        a -= 1
        convar.notify_all()
    });

    //Escribe
    std::thread::spawn(move || {
        let a = convar.wait_while(variables.lock().unwrap(), |lock| {!*lock[0] || !*lock[1]})
        
        convar.notify_all()
    });

}

// fn pseudo_fumadores() {
//     //Fuego cigarrillo tabaco

//     let variables = Arc::new(Mutex::new((false, false, false)));
//     let convar = Arc::new(Convar::new());
//     //Fuego
//     std::thread::spawn(move || {
//         convar.wait_while(variables.lock().unwrap(), |lock| {!*lock[1] || !*lock[2]})
//         //Poner lo que tome en false
//     });

//     //Cigarrillo
//     std::thread::spawn(move || {
//         convar.wait_while(variables.lock().unwrap(), |lock| {!*lock[0] || !*lock[2]})
//         //Poner lo que tome en false
//     });

//     algo 
//         .map
//         .filter
//         .flattern
//         .algo
//         .tuvieja
//         .rustesunaverga
//         .femboy

//     //tabaco
//     std::thread::spawn(move || {
//         convar.wait_while(variables.lock().unwrap(), |lock| {!*lock[0] || !*lock[1]})
//         //Poner lo que tome en false
//     });

//     //mozo
//     std::thread::spawn(move || {
//         convar.wait_while(variables.lock(), move | | {"todo este en false"})};
//         let valor1 = random(1, 3)
//         let valor2 = random(1, 3)

//         variables.lock().unwrap = //settear lo que corresponde en true
//         convar.notify_all()
//     });
// }

// fn psuedo_barbero() {
//     std::thread::spawn(move || {
//         //Espero a que haya un cliente disponible
//         cliente_disponible.wait();

//         //Le aviso al cliente que le termine
//         //de cortar el pelo
//         cortando_pelo.release();

//         //Indico que estoy disponible
//         barbero_disponible.release();
//     });

//     for i in 0..200 {
//         std::thread::spawn(move || {
//             //Indico que estoy en la sala de espera
//             cliente_disponible.release()

//             //Espero a que el barbero este disponible
//             barbero_disponible.wait()

//             //Espero a que me termine de cortar el pelo
//             cortando_pelo.wait()
//         })
//     }
// }
fn main() {
    barbero();
}