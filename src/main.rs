/*
Un viejo banquero retirado se mudó a vivir al campo lejos de toda la tecnología. 
Para vivir, invierte la plata que hizo durante sus años de trabajo mediante 
los amigos que tiene en diversos fondos de inversión. Al inicio de cada semana 
les envía por correo el dinero para invertir a sus amigos; luego espera hasta 
el final de la semana a que le envíen a su buzón el resultado de esas inversiones.

Modelar la situación planteada en Rust, considerando que:
- A todos los amigos les envía el mismo monto
- Que el dinero resultante lo vuelve a invertir la próxima semana
- Que las inversiones pueden dar una ganancia entre -50% y 50% de lo invertido.
*/
use std::env;
use rand::prelude::*;

/// Se invoca pasando por argumento el monto a dividir, la cantidad de amigos y 
/// el numero de semanas que invertira:
/// cargo run <monto> <#amigos> <#semanas>
/// Devuelve el monto obtenido, teniendo en cuenta lo invertido.
fn main() {
  let args:Vec<String> = env::args().collect();
  let monto: f32 = args[1].parse().unwrap_or(0.0);
  let amigos = args[2].parse().unwrap_or(0);
  let semanas = args[3].parse().unwrap_or(0);
  let total = invertir_con_amigos(monto, amigos, semanas, 1);
  println!("Total obtenido: {}", total);
}

/// Invierte el monto dado dividiendolo equitativamente entre el numero de amigos
/// dado, reinvirtiendolo el numero de veces indicado por semanas.
fn invertir_con_amigos(monto: f32, amigos: i32, semanas: i32, semana: i32) -> f32 {
  let inversion = monto/(amigos as f32);
  let mut total = 0.0;
  for i in 0..amigos {
    let ganado = invertir(inversion);
    total += ganado;
    println!("Amigo {} en la semana {} obtuvo: {}", i, semana, ganado);
  }
  println!("Ingreso total semana {}: {}", semana, total);
  if semana != semanas{
    total = invertir_con_amigos(total, amigos, semanas, semana + 1);
  }
  total
}

/// Invierte i, generando una ganancia entre -50% y 50% de i.
fn invertir(i: f32) -> f32 {
  let mut rng = thread_rng();
  let ganancia = rng.gen_range(-0.5, 0.5);
  return i + i*ganancia;
}