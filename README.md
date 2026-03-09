# 📋 Sistema de Consultoría en Solana

![banner](./images/consultoria.jfif)

> Smart Contract en Solana para la gestión descentralizada de citas y consultorías en la blockchain.

---

## ¿Qué es Sistema de Consultoría?

Sistema de Consultoría es un programa desplegado en la **blockchain de Solana** que permite a consultores profesionales gestionar sus citas de forma **descentralizada e inmutable**.

Cada consultoría queda almacenada como una cuenta única (**PDA - Program Derived Address**) que contiene:

* El **nombre de la consultoría**
* El **propietario (wallet del consultor)**
* Un **registro de citas con clientes y fechas**

Esto permite que la información sea **segura, transparente y verificable en la blockchain**.

El proyecto fue desarrollado utilizando **Rust** y el framework **Anchor**, siguiendo las prácticas recomendadas para el desarrollo de **Smart Contracts en Solana**.

---

## Instrucciones del Programa

| Instrucción         | Acción     | Descripción                                     |
| ------------------- | ---------- | ----------------------------------------------- |
| `crear_consultoria` | **CREATE** | Registra una nueva consultoría en la blockchain |
| `ver_citas`         | **READ**   | Visualiza todas las citas registradas           |
| `agendar_cita`      | **CREATE** | Agrega una nueva cita con cliente y fecha       |
| `actualizar_fecha`  | **UPDATE** | Modifica la fecha de una cita existente         |
| `eliminar_cita`     | **DELETE** | Elimina una cita específica del registro        |

---

## Estructura de Datos

Cada consultoría se almacena en su propia cuenta **PDA** con los siguientes campos:

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
```

Las cuentas se derivan mediante **Program Derived Addresses (PDA)**, lo que garantiza que cada consultoría tenga una dirección única dentro de la blockchain.

---

## Cómo Ejecutarlo

### 1. Importar en Solana Playground

Copia el enlace de tu repositorio y ábrelo en **Solana Playground**:

```
https://beta.solpg.io/github.com/JECHAPE08/Sistema-Consultoria-Solana
```

Haz clic en **Import** y asigna un nombre al proyecto.

![import](./images/import.png)

---

### 2. Conectar Wallet

Haz clic en **Not Connected** en la parte inferior izquierda para conectarte a la **Devnet** y crear tu wallet de pruebas.

![wallet](./images/wallet.png)

Solicita SOL de prueba en la terminal:

```bash
solana airdrop 2
```

Esto te permitirá pagar las comisiones necesarias para desplegar el programa.

---

### 3. Compilar y Desplegar

1. Haz clic en **Build** para compilar el programa
2. Espera la marca verde de compilación exitosa
3. Haz clic en **Deploy** para desplegar el contrato en **Devnet**

Cuando finalice aparecerá el mensaje:

```
Deployment successful
```

---

### 4. Ejecutar las Pruebas

En la terminal de **Solana Playground** ejecuta:

```bash
run
```

Esto ejecutará el archivo `client/client.ts`, el cual prueba las funcionalidades principales del sistema:

* Crear una consultoría
* Agendar una cita
* Consultar citas registradas
* Actualizar la fecha de una cita
* Eliminar una cita

---

## Tecnologías

| Herramienta           | Uso                                      |
| --------------------- | ---------------------------------------- |
| **Rust**              | Lógica del Smart Contract                |
| **Anchor Framework**  | Framework para desarrollo en Solana      |
| **TypeScript**        | Cliente para interactuar con el contrato |

---

## Autor

Desarrollado por **JECHAPE08**
