# 📋 Sistema de Consultoría en Solana

![banner](./images/consultoria.jfif)

---

## ¿Qué es Sistema de Consultoría?

Sistema de Consultoría es un programa desplegado en la **blockchain de Solana** que permite a consultores profesionales gestionar sus citas de forma inmutable y descentralizada. Cada consultoría queda almacenada como una cuenta única (PDA) con su **nombre**, **propietario** y un **registro completo de citas** con clientes y fechas.

Desarrollado con **Rust** y el framework **Anchor** como parte del Bootcamp de certificación de la **Solana Foundation**.

---

## Instrucciones del Programa

| Instrucción        | Acción     | Descripción                                            |
| ------------------ | ---------- | ------------------------------------------------------ |
| `crear_consultoria`| **CREATE** | Registra una nueva consultoría en la blockchain        |
| `ver_citas`        | **READ**   | Visualiza todas las citas agendadas                    |
| `agendar_cita`     | **CREATE** | Agrega una nueva cita con cliente y fecha              |
| `actualizar_fecha` | **UPDATE** | Modifica la fecha de una cita existente                |
| `eliminar_cita`    | **DELETE** | Elimina una cita específica del registro               |

---

## Estructura de Datos

Cada consultoría se almacena en su propia cuenta PDA con los siguientes campos:

```rust
pub struct Consultoria {
    pub owner: Pubkey,        // Wallet del consultor propietario
    pub nombre: String,       // "Consultoría Legal", "Asesoría Financiera"
    pub citas: Vec<Cita>,     // Vector dinámico de citas (máx 20)
}

pub struct Cita {
    pub cliente: String,      // "Juan Pérez", "María García"
    pub fecha: String,        // "2024-12-15", "15/12/2024 10:30"
}
