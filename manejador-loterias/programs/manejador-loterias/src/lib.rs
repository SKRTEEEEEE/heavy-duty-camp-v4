use anchor_lang::prelude::*;
use crate::instrucciones::*;

// treamos los modulos al scope
mod colecciones;
mod instrucciones;

declare_id!("GVEW7KHyM4knGLYQXyME1gzAxvtDGP9oPoVcppXYoNwH");

#[program]
mod manejador_loterias {
    use super::*;

    // creamos la instruccion crear evento
    pub fn crear_loteria(
        ctx: Context<CrearLoteria>,
        id: String,
        nombre: String,
        descripcion: String,
        precio_token: f64,
        tipo_loteria: f64,
    ) -> Result<()> {
        instrucciones::crear_loteria(ctx, id, nombre, descripcion, precio_token, tipo_loteria)?;
        Ok(())
    }

    pub fn eliminar_loteria(ctx: Context<EliminarLoteria>)->Result<()>{
        instrucciones::eliminar_loteria(ctx)?;
        Ok(())    
    }

    pub fn parar_loteria(ctx: Context<PararLoteria>)->Result<()>{
        instrucciones::parar_loteria(ctx)?;
        Ok(())
    }
}
