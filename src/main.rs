use std::io;
use rand::Rng;
/*fn trustaligula(hola:i32, peru:i32)->f64{
    return (hola/peru) as f64;
} */
fn main() {
  /*  println!("rey pasame un numerin para estresar todo la computadora");
    let mut bitli = String::new();
    io::stdin()
        .read_line(&mut bitli)
        .expect("que ha pasao tiio");
*/

    //println!("Hello, world!");
let marimos = rand::rng().random_range(1..=500);
   println!("peru te presenta este numero {}\n", marimos);
   loop{
   println!("epa rey necesito que me digas un numeraso ");
       let mut bit = String::new();
   io::stdin()
       .read_line(&mut bit)
       .expect("bradar what is this comment");
    let elimera: i32 = bit
    .trim()
    .parse()
    .expect("bradar what is dis ");
       match elimera{
           m if m>marimos => println!("epa tio no has llegao,te has pasao"),
           m if m<marimos => println!("epa tio te has atrasao"),
           m if m == marimos =>{

            println!("has llegao tio felicidades");
            break;
           },
           _ => println!("viva peru "),

       }
 
   }
}
/*    if elimera == marimos{
        println!("lo lograste felicidades crack el numero paraz adivinar era {}", marimos);
        break;
    }
else if elimera >marimos {
    println!("te has pasao tiooo");
}
else if elimera<marimos{
    println!("muy cerca tio siento el ardor al lado mio co pero te has atrasao ");

}

    else{
        println!("has fallao tio intenta de nuevo");
    }
*/

   //}

        /*let aimon = bitli
    .trim()
    .parse()
    .expect("hemos explotao tio");
     let mut  emriminos =0;
    for emos in 0..=aimon{
        let semillazo = rand::rng().random_range(1..=3229);
       // println!("epa aqui tienes el semillazo {}",semillazo);
    emriminos +=semillazo;
    println!("epa estamos en: {} y además esto es toda la data junta: {}", emos,emriminos);
    println!("la semilla actual es:{}", semillazo);
        if emos == aimon{
            let trustaresultado :f64 = trustaligula(emriminos, aimon);
            println!("el promedio de toda esta locura es {}, y la locura sin nada es {}", trustaresultado,emriminos );
        }

    } 
*/ 


//}
