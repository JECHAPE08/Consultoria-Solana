use anchor_lang::prelude::*;

declare_id!("8QJBNZWYnE6frA7FLGnVYC2NkevhZL33AEEY91ipsEsH");

#[program]
pub mod sistema_consultoria {
    use super::*;

    pub fn crear_consultoria(ctx: Context<NuevaConsultoria>, nombre: String) -> Result<()> {
        let owner_id = ctx.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let citas: Vec<Cita> = Vec::new();

        ctx.accounts.consultoria.set_inner(Consultoria {
            owner: owner_id,
            nombre,
            citas,
        });

        Ok(())
    }

    pub fn agendar_cita(ctx: Context<NuevaCita>, cliente: String, fecha: String) -> Result<()> {
        require!(
            ctx.accounts.consultoria.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        let cita = Cita { cliente, fecha };

        ctx.accounts.consultoria.citas.push(cita);

        Ok(())
    }

    pub fn ver_citas(ctx: Context<NuevaCita>) -> Result<()> {
        require!(
            ctx.accounts.consultoria.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        msg!("Lista de citas: {:#?}", ctx.accounts.consultoria.citas);
        Ok(())
    }

    pub fn eliminar_cita(ctx: Context<NuevaCita>, cliente: String) -> Result<()> {
        require!(
            ctx.accounts.consultoria.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        let citas = &mut ctx.accounts.consultoria.citas;

        for i in 0..citas.len() {
            if citas[i].cliente == cliente {
                citas.remove(i);
                msg!("Cita de {} eliminada!", cliente);
                return Ok(());
            }
        }

        Err(ErrorPersonalizado::CitaNoExiste.into())
    }

    pub fn actualizar_fecha(
        ctx: Context<NuevaCita>,
        cliente: String,
        nueva_fecha: String,
    ) -> Result<()> {
        require!(
            ctx.accounts.consultoria.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        let citas = &mut ctx.accounts.consultoria.citas;

        for i in 0..citas.len() {
            if citas[i].cliente == cliente {
                citas[i].fecha = nueva_fecha;
                msg!("Fecha actualizada para cita de: {}", cliente);
                return Ok(());
            }
        }

        Err(ErrorPersonalizado::CitaNoExiste.into())
    }
}

#[error_code]
pub enum ErrorPersonalizado {
    #[msg("Error, no eres el propietario de la consultoria")]
    NoAutorizado,
    #[msg("Error, la cita no existe")]
    CitaNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Consultoria {
    pub owner: Pubkey,

    #[max_len(30)]
    pub nombre: String,

    #[max_len(20)]
    pub citas: Vec<Cita>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Cita {
    #[max_len(30)]
    pub cliente: String,

    #[max_len(15)]
    pub fecha: String,
}

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

#[derive(Accounts)]
pub struct NuevaCita<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub consultoria: Account<'info, Consultoria>,
}
