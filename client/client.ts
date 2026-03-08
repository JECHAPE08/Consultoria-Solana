// Script para interactuar con el sistema de consultoria

function getConsultoriaPDA() {
  const [consultoriaPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("consultoria"), pg.wallet.publicKey.toBuffer()],
    pg.program.programId
  );
  return consultoriaPda;
}

const consultoriaPda = getConsultoriaPDA();
console.log("Direccion PDA de la consultoria:", consultoriaPda.toBase58());

// Funcion 1: Crear Consultoria
async function crearConsultoria(nombre: string) {
  try {
    console.log(`\nCreando consultoria: "${nombre}"...`);

    const tx = await pg.program.methods
      .crearConsultoria(nombre)
      .accounts({
        owner: pg.wallet.publicKey,
        consultoria: consultoriaPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Consultoria creada exitosamente");
    console.log("Transaction:", tx);
  } catch (e) {
    console.log("La consultoria ya existe");
  }
}

// Funcion 2: Agregar Cita (evita duplicados)
async function agregarCita(cliente: string, fecha: string) {
  try {
    // Verificar si la cita ya existe
    const consultoriaData = await pg.program.account.consultoria.fetch(
      consultoriaPda
    );
    const citaExistente = consultoriaData.citas.find(
      (c) => c.cliente === cliente && c.fecha === fecha
    );

    if (citaExistente) {
      console.log(`\nLa cita para ${cliente} el ${fecha} ya existe`);
      return;
    }

    console.log(`\nAgendando cita para: ${cliente} el ${fecha}...`);

    const tx = await pg.program.methods
      .agendarCita(cliente, fecha)
      .accounts({
        owner: pg.wallet.publicKey,
        consultoria: consultoriaPda,
      })
      .rpc();

    console.log("Cita agendada exitosamente");
    console.log("Transaction:", tx);
  } catch (e) {
    console.error("Error al agendar cita:", e.message);
  }
}

// Funcion 3: Ver todas las citas
async function verCitas() {
  try {
    console.log("\nObteniendo lista de citas...");

    const consultoriaData = await pg.program.account.consultoria.fetch(
      consultoriaPda
    );

    console.log("=== INFORMACION DE CONSULTORIA ===");
    console.log("Nombre:", consultoriaData.nombre);
    console.log("Dueño:", consultoriaData.owner.toBase58());
    console.log("Total citas:", consultoriaData.citas.length);

    if (consultoriaData.citas.length === 0) {
      console.log("No hay citas agendadas");
    } else {
      console.log("=== LISTA DE CITAS ===");
      consultoriaData.citas.forEach((cita, index) => {
        console.log(`\nCita #${index + 1}:`);
        console.log(`  Cliente: ${cita.cliente}`);
        console.log(`  Fecha: ${cita.fecha}`);
      });
    }

    return consultoriaData.citas;
  } catch (e) {
    console.error("Error al ver citas:", e.message);
  }
}

// Funcion 4: Editar fecha de una cita
async function editarCita(cliente: string, nuevaFecha: string) {
  try {
    console.log(
      `\nEditando cita de: ${cliente} a nueva fecha: ${nuevaFecha}...`
    );

    const tx = await pg.program.methods
      .actualizarFecha(cliente, nuevaFecha)
      .accounts({
        owner: pg.wallet.publicKey,
        consultoria: consultoriaPda,
      })
      .rpc();

    console.log("Cita editada exitosamente");
    console.log("Transaction:", tx);

    // Verificar el cambio
    const consultoriaData = await pg.program.account.consultoria.fetch(
      consultoriaPda
    );
    const citaEditada = consultoriaData.citas.find(
      (c) => c.cliente === cliente
    );
    if (citaEditada) {
      console.log("Nueva fecha verificada:", citaEditada.fecha);
    }
  } catch (e) {
    console.error("Error al editar cita:", e.message);
  }
}

// Funcion 5: Eliminar cita
async function eliminarCita(cliente: string, fecha?: string) {
  try {
    // Si se especifica fecha, buscar cliente+fecha
    if (fecha) {
      const consultoriaData = await pg.program.account.consultoria.fetch(
        consultoriaPda
      );
      const citaExistente = consultoriaData.citas.find(
        (c) => c.cliente === cliente && c.fecha === fecha
      );

      if (!citaExistente) {
        console.log(`\nNo se encontro cita para ${cliente} el ${fecha}`);
        return;
      }
    }

    console.log(`\nEliminando cita de: ${cliente}...`);

    const tx = await pg.program.methods
      .eliminarCita(cliente)
      .accounts({
        owner: pg.wallet.publicKey,
        consultoria: consultoriaPda,
      })
      .rpc();

    console.log("Cita eliminada exitosamente");
    console.log("Transaction:", tx);
  } catch (e) {
    console.error("Error al eliminar cita:", e.message);
  }
}

// Funcion 6: Limpiar todas las citas (para reiniciar)
async function limpiarTodasLasCitas() {
  try {
    const consultoriaData = await pg.program.account.consultoria.fetch(
      consultoriaPda
    );
    const citas = [...consultoriaData.citas];

    for (const cita of citas) {
      console.log(`\nEliminando cita de: ${cita.cliente}...`);
      await pg.program.methods
        .eliminarCita(cita.cliente)
        .accounts({
          owner: pg.wallet.publicKey,
          consultoria: consultoriaPda,
        })
        .rpc();
    }

    console.log("\nTodas las citas eliminadas");
  } catch (e) {
    console.error("Error al limpiar citas:", e.message);
  }
}

// EJEMPLO DE USO
(async () => {
  console.log("SISTEMA DE CONSULTORIA - OPERACIONES CRUD");

  // Crear consultoria (si no existe)
  await crearConsultoria("PSIQUECONSULTORES");

  // LIMPIAR CITAS
  // await limpiarTodasLasCitas();

  // Agregar citas (evita duplicados)
  await agregarCita("Jesus Perez", "15-03-2026");
  await agregarCita("Gerardo Chavez", "16-03-2026");
  await agregarCita("Juan Lopez", "17-03-2026");

  // Ver citas
  await verCitas();

  // Editar cita de Jesus Perez (NO Juan Perez)
  await editarCita("Jesus Perez", "20-03-2026");

  // Ver citas despues de editar
  await verCitas();

  // Eliminar cita de Gerardo Chavez (NO Maria Garcia)
  await eliminarCita("Gerardo Chavez");

  // Ver citas finales
  await verCitas();
})();
