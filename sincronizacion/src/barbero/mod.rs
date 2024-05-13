use std_semaphore::Semaphore;
use std::sync::Arc;

const CANTIDAD_DE_CLIENTES: usize = 200;

pub fn barbero() {
    //Se va a modelar con threads para cada entidad, uno para cada cliente y uno para el barbero

    //Para modelar la cantidad de clientes que tengo, y que el barbero sepa si tiene a alguien para
    //cortarle el pelo o no, tengo un semaforo, que cada vez que hay un nuevo cliente, le hace un release()
    //sumandole 1, e indicando que efectivamente esta en la sala de espera
    let sala_de_espera = Arc::new(Semaphore::new(0));

    //Solo le puedo cortar el pelo a uno a la vez, asi que todos los clientes tendrian que esperar a poder
    //adquirir ese recurso (la silla del barbero), para eso uso un semaforo que me va a indicar si la silla
    //esta libre o no, cuando un cliente la toma, va a quedar en 0, haciendo que el resto de clientes esten
    //esperando por ella. El que indica cuando se libera la silla va a tener que ser el barbero, o sea que 
    //cuando el barbero termine de realizar el corte de pelo, va a liberar la silla 
    let silla_de_barbero = Arc::new(Semaphore::new(0));

    //Cada cliente tiene que saber cuando le terminan de cortar el pelo, para ello uso otro semaforo, al cual el barbero
    //va a indicar que termino de cortar el pelo, y el cliente va a estar esperando a que el barbero indique que ya termino
    //de cortarle
    let corte_listo = Arc::new(Semaphore::new(0));

    let sala_de_espera_barbero = sala_de_espera.clone();
    let silla_de_barbero_barbero = silla_de_barbero.clone();
    let corte_listo_barbero = corte_listo.clone();

    
    
    for i in 1..=CANTIDAD_DE_CLIENTES {
        let sala_de_espera_cliente = sala_de_espera.clone();
        let silla_de_barbero_cliente = silla_de_barbero.clone();
        let corte_listo_cliente = corte_listo.clone();
        
        std::thread::spawn(move || {
            println!("[CLIENTE {}] se sento en la sala de espera", i);
            //Le indico al barbero que llegue a la sala de espera
            sala_de_espera_cliente.release();
            
            println!("[CLEINTE {}] esta esperando al barbero", i);
            //Intento sentarme en la silla del barbero, si no puedo porque hay alguien mas,
            //espero a que se libere
            silla_de_barbero_cliente.acquire();
            
            println!("[CLIENTE {}] le estan cortando el pelo", i);
            //Tengo que esperar a que el barbero termine de cortarme el pelo
            corte_listo_cliente.acquire();
            
            println!("[CLIENTE {}] le terminaron de cortar el pelo", i);
        });
    }
    
    //Se puede poner arriba y da igual, en verdad tiene mas sentido, pero
    //asi el output es mas lindo
    let handle = std::thread::spawn(move || {
        loop {
            println!("[BARBERO] Esperando clientes");
            //Atiendo a un cliente si es que tengo alguno, sino me quedo esperando por clientes
            sala_de_espera_barbero.acquire();

            println!("[BARBERO] Voy a cortarle el pelo a alguien");
            //Como obtuve un cliente, ahora le corto el pelo
            std::thread::sleep(std::time::Duration::from_secs(1));

            println!("[BARBERO] Termine de cortar el pelo");
            //Una vez que termine de cortar el pelo, tengo que indicarle al cliente que termine de cortarle el pelo
            //y tengo que avisarle al resto de clientes esperandome que puedo atender a alguien mas
            corte_listo_barbero.release();
            silla_de_barbero_barbero.release();
        }
    });

    //Si no se hace esto, cuando termina la funcion (tira los 200 clientes)
    //muere el thread del barbero de una
    handle.join().unwrap();
}
