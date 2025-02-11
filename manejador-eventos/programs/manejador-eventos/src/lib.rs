
use anchor_lang::prelude::*;
use crate::instrucciones::*;

// treamos los modulos al scope
mod colecciones;
mod instrucciones;
mod utilidades;


declare_id!("C9vNVb15tnhMrJbZQFudbsKfUE8zL1hgsuZkqWz9GkYJ");

#[program]
mod manejador_eventos {
    use super::*;

    // creamos la instruccion crear evento
    pub fn crear_evento(
        ctx: Context<CrearEvento>,
        id: String,
        nombre: String,
        descripcion: String,
        precio_entrada: f64,
        precio_token: f64,
    ) -> Result<()> {
        instrucciones::crear_evento(ctx, id, nombre, descripcion, precio_entrada, precio_token)?;
        Ok(())
    }
    pub fn eliminar_evento(ctx: Context<EliminarEvento>) -> Result<()> {
        instrucciones::eliminar_evento(ctx)?;
        Ok(())
    }
    pub fn finalizar_evento(ctx: Context<FinalizarEvento>) -> Result<()> {
        instrucciones::finalizar_evento(ctx)?;
        Ok(())
    }
}

