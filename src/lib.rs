use anchor_lang::prelude::*;
declare_id!("");
#[program]
pub mod sistema_consultoria {
    use super::*;

    // Función para crear una nueva consultoría
    // Parámetros:
    // - nombre: Nombre de la consultoría
    pub fn crear_consultoria(ctx: Context<NuevaConsultoria>, nombre: String) -> Result<()> {
        let owner_id = ctx.accounts.owner.key();

        // Inicializa un vector vacío para las citas
        let citas: Vec<Cita> = Vec::new();

        // Establece los datos de la cuenta consultoria
        ctx.accounts.consultoria.set_inner(Consultoria {
            owner: owner_id,
            nombre,
            citas,
        });

        Ok(())
    }

    // Función para agendar una nueva cita
    // Parámetros:
    // - cliente: Nombre del cliente
    // - fecha: Fecha de la cita
    pub fn agendar_cita(ctx: Context<NuevaCita>, cliente: String, fecha: String) -> Result<()> {
        // Verifica que el firmante sea el propietario de la consultoría
        require!(
            ctx.accounts.consultoria.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        // Crea una nueva instancia de Cita
        let cita = Cita { cliente, fecha };

        // Agrega la cita al vector de citas de la consultoría
        ctx.accounts.consultoria.citas.push(cita);

        Ok(())
    }

    // Función para visualizar todas las citas
    // Parámetros:
    pub fn ver_citas(ctx: Context<NuevaCita>) -> Result<()> {
        // Verifica que el firmante sea el propietario de la consultoría
        require!(
            ctx.accounts.consultoria.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        // Muestra en log la lista completa de citas con formato debug
        msg!("Lista de citas: {:#?}", ctx.accounts.consultoria.citas);
        Ok(())
    }

    // Función para eliminar una cita por nombre de cliente
    // Parámetros:
    // - cliente: Nombre del cliente cuya cita se eliminará
    pub fn eliminar_cita(ctx: Context<NuevaCita>, cliente: String) -> Result<()> {
        // Verifica que el firmante sea el propietario de la consultoría
        require!(
            ctx.accounts.consultoria.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        // Obtiene una referencia mutable al vector de citas
        let citas = &mut ctx.accounts.consultoria.citas;

        // Itera sobre las citas para encontrar la del cliente especificado
        for i in 0..citas.len() {
            if citas[i].cliente == cliente {
                citas.remove(i); // Elimina la cita encontrada
                msg!("Cita de {} eliminada!", cliente);
                return Ok(());
            }
        }

        // Si no encuentra la cita, retorna error
        Err(ErrorPersonalizado::CitaNoExiste.into())
    }

    // Función para actualizar la fecha de una cita existente
    // Parámetros:
    // - cliente: Nombre del cliente cuya cita se actualizará
    // - nueva_fecha: Nueva fecha para la cita
    pub fn actualizar_fecha(
        ctx: Context<NuevaCita>,
        cliente: String,
        nueva_fecha: String,
    ) -> Result<()> {
        // Verifica que el firmante sea el propietario de la consultoría
        require!(
            ctx.accounts.consultoria.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        // Obtiene una referencia mutable al vector de citas
        let citas = &mut ctx.accounts.consultoria.citas;

        // Itera sobre las citas para encontrar la del cliente especificado
        for i in 0..citas.len() {
            if citas[i].cliente == cliente {
                citas[i].fecha = nueva_fecha; // Actualiza la fecha
                msg!("Fecha actualizada para cita de: {}", cliente); // Log de confirmación
                return Ok(());
            }
        }

        // Si no encuentra la cita, retorna error
        Err(ErrorPersonalizado::CitaNoExiste.into())
    }
}

// Definición de errores personalizados para el programa
#[error_code]
pub enum ErrorPersonalizado {
    #[msg("Error, no eres el propietario de la consultoria")]
    NoAutorizado, // Error cuando alguien no autorizado intenta una acción
    #[msg("Error, la cita no existe")]
    CitaNoExiste, // Error cuando se busca una cita que no existe
}

// Estructura de cuenta para la Consultoría
#[account]
#[derive(InitSpace)]
pub struct Consultoria {
    pub owner: Pubkey, // Clave pública del propietario

    #[max_len(30)]
    pub nombre: String, // Nombre de la consultoría (máx 30 caracteres)

    #[max_len(20)]
    pub citas: Vec<Cita>, // Vector de citas (máx 20 citas)
}

// Estructura para representar una Cita
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Cita {
    #[max_len(30)]
    pub cliente: String, // Nombre del cliente (máx 30 caracteres)

    #[max_len(15)]
    pub fecha: String, // Fecha de la cita (máx 15 caracteres)
}

// Contexto para crear una nueva consultoría
#[derive(Accounts)]
pub struct NuevaConsultoria<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,                                  
        payer = owner,                           
        space = 8 + Consultoria::INIT_SPACE,    
        seeds = [b"consultoria", owner.key().as_ref()], 
        bump                                     
    )]
    pub consultoria: Account<'info, Consultoria>,

    pub system_program: Program<'info, System>,
}

// Contexto para operaciones con citas
#[derive(Accounts)]
pub struct NuevaCita<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub consultoria: Account<'info, Consultoria>,
}
