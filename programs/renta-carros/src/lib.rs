use anchor_lang::prelude::*;

declare_id!("5Ycogw1FoNJXFX3hsgtFZ1Pemz58ZUXxdEdeRAx96t26");

#[program]
pub mod renta_carros {
    use super::*;

    pub fn crear_agencia(ctx: Context<NuevaAgencia>, nombre: String) -> Result<()> {
        let owner = ctx.accounts.owner.key();
        let flota: Vec<Carro> = Vec::new();

        ctx.accounts.agencia.set_inner(Agencia {
            owner,
            nombre,
            flota,
        });

        Ok(())
    }

    pub fn registrar_carro(ctx: Context<GestionCarro>, marca: String, modelo: String, año: u16) -> Result<()> {
        let nuevo_carro = Carro {
            marca,
            modelo,
            año,
            disponible: true,
        };

        ctx.accounts.agencia.flota.push(nuevo_carro);
        msg!("Carro registrado exitosamente");

        Ok(())
    }

    pub fn ver_catalogo(ctx: Context<GestionCarro>) -> Result<()> {
        msg!(
            "Catálogo de la agencia {}: {:#?}",
            ctx.accounts.agencia.nombre,
            ctx.accounts.agencia.flota
        );
        Ok(())
    }

    pub fn eliminar_carro(ctx: Context<GestionCarro>, modelo: String) -> Result<()> {
        let flota = &mut ctx.accounts.agencia.flota;

        if let Some(index) = flota.iter().position(|c| c.modelo == modelo) {
            flota.remove(index);
            msg!("Carro modelo {} eliminado del catálogo", modelo);
            return Ok(());
        }

        Err(Errores::CarroNoEncontrado.into())
    }

    pub fn rentar_o_devolver(ctx: Context<GestionCarro>, modelo: String) -> Result<()> {
        let flota = &mut ctx.accounts.agencia.flota;

        for carro in flota.iter_mut() {
            if carro.modelo == modelo {
                carro.disponible = !carro.disponible;
                let estado = if carro.disponible { "disponible" } else { "rentado" };
                
                msg!("El carro {} ahora está {}", modelo, estado);
                return Ok(());
            }
        }

        Err(Errores::CarroNoEncontrado.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("No tienes permisos para modificar esta agencia.")]
    NoAutorizado,

    #[msg("El carro especificado no se encuentra en la flota.")]
    CarroNoEncontrado,
}

#[account]
#[derive(InitSpace)]
pub struct Agencia {
    pub owner: Pubkey,

    #[max_len(50)]
    pub nombre: String,

    #[max_len(15)] // Capacidad para 15 carros
    pub flota: Vec<Carro>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Carro {
    #[max_len(30)]
    pub marca: String,

    #[max_len(30)]
    pub modelo: String,

    pub año: u16,

    pub disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaAgencia<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Agencia::INIT_SPACE,
        seeds = [b"agencia", owner.key().as_ref()],
        bump
    )]
    pub agencia: Account<'info, Agencia>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionCarro<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner @ Errores::NoAutorizado // Validación de seguridad
    )]
    pub agencia: Account<'info, Agencia>,
}