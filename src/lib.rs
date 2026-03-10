use anchor_lang::prelude::*;

declare_id!("51qM7KqFoQGd3xiyGnVKfSAYTkR4Ubh4mZaGzKjTPRHw");

#[program]
pub mod proof_of_learning {

    use super::*;

    ///////////////////////////
    // Crear Curso
    ///////////////////////////

    pub fn crear_curso(context: Context<NuevoCurso>, nombre: String) -> Result<()> {

        let instructor = context.accounts.instructor.key();

        let certificados = Vec::<Pubkey>::new();

        context.accounts.curso.set_inner(Course {
            instructor,
            nombre: nombre.clone(),
            certificados,
        });

        msg!("Curso {} creado!", nombre);

        Ok(())
    }

    ///////////////////////////
    // CREATE certificado
    ///////////////////////////

    pub fn emitir_certificado(
        context: Context<NuevoCertificado>,
        titulo: String
    ) -> Result<()> {

        let certificado = Certificate {
            student: context.accounts.student.key(),
            course: context.accounts.curso.key(),
            instructor: context.accounts.instructor.key(),
            titulo: titulo.clone(),
            issued_at: Clock::get()?.unix_timestamp,
            activo: true,
        };

        context.accounts.certificado.set_inner(certificado);

        context
            .accounts
            .curso
            .certificados
            .push(context.accounts.certificado.key());

        msg!("Certificado emitido para {}", context.accounts.student.key());

        Ok(())
    }

    ///////////////////////////
    // UPDATE certificado
    ///////////////////////////

    pub fn actualizar_certificado(
        context: Context<ModificarCertificado>,
        nuevo_titulo: String
    ) -> Result<()> {

        require!(
            context.accounts.curso.instructor == context.accounts.instructor.key(),
            ErrorCode::NoEresInstructor
        );

        let certificado = &mut context.accounts.certificado;

        certificado.titulo = nuevo_titulo;

        msg!("Certificado actualizado");

        Ok(())
    }

    ///////////////////////////
    // DELETE certificado
    ///////////////////////////

    pub fn eliminar_certificado(
        context: Context<EliminarCertificado>
    ) -> Result<()> {

        require!(
            context.accounts.curso.instructor == context.accounts.instructor.key(),
            ErrorCode::NoEresInstructor
        );

        let curso = &mut context.accounts.curso;

        require!(
            curso.certificados.contains(&context.accounts.certificado.key()),
            ErrorCode::CertificadoNoExiste
        );

        let pos = curso
            .certificados
            .iter()
            .position(|&x| x == context.accounts.certificado.key())
            .ok_or(ErrorCode::CertificadoNoExiste)?;

        curso.certificados.remove(pos);

        msg!("Certificado eliminado");

        Ok(())
    }
}

///////////////////////////
// ERRORES
///////////////////////////

#[error_code]
pub enum ErrorCode {

    #[msg("No eres el instructor del curso")]
    NoEresInstructor,

    #[msg("El certificado no existe")]
    CertificadoNoExiste,
}

///////////////////////////
// CUENTAS
///////////////////////////

#[account]
#[derive(InitSpace)]
pub struct Course {

    pub instructor: Pubkey,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(100)]
    pub certificados: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Certificate {

    pub student: Pubkey,

    pub course: Pubkey,

    pub instructor: Pubkey,

    #[max_len(60)]
    pub titulo: String,

    pub issued_at: i64,

    pub activo: bool,
}

///////////////////////////
// CONTEXTOS
///////////////////////////

///////////////////////////
// Crear Curso
///////////////////////////

#[derive(Accounts)]
#[instruction(nombre:String)]
pub struct NuevoCurso<'info> {

    #[account(mut)]
    pub instructor: Signer<'info>,

    #[account(
        init,
        payer = instructor,
        space = 8 + Course::INIT_SPACE,
        seeds = [
            b"course",
            nombre.as_bytes(),
            instructor.key().as_ref()
        ],
        bump
    )]
    pub curso: Account<'info, Course>,

    pub system_program: Program<'info, System>,
}

///////////////////////////
// Emitir Certificado
///////////////////////////

#[derive(Accounts)]
pub struct NuevoCertificado<'info> {

    #[account(mut)]
    pub instructor: Signer<'info>,

    #[account(mut)]
    pub student: Signer<'info>,

    #[account(mut)]
    pub curso: Account<'info, Course>,

    #[account(
        init,
        payer = student,
        space = 8 + Certificate::INIT_SPACE,
        seeds = [
            b"certificate",
            student.key().as_ref(),
            curso.key().as_ref()
        ],
        bump
    )]
    pub certificado: Account<'info, Certificate>,

    pub system_program: Program<'info, System>,
}

///////////////////////////
// Actualizar Certificado
///////////////////////////

#[derive(Accounts)]
pub struct ModificarCertificado<'info> {

    pub instructor: Signer<'info>,

    #[account(mut)]
    pub certificado: Account<'info, Certificate>,

    #[account(mut)]
    pub curso: Account<'info, Course>,
}

///////////////////////////
// Eliminar Certificado
///////////////////////////

#[derive(Accounts)]
pub struct EliminarCertificado<'info> {

    pub instructor: Signer<'info>,

    #[account(
        mut,
        close = instructor
    )]
    pub certificado: Account<'info, Certificate>,

    #[account(mut)]
    pub curso: Account<'info, Course>,
}
