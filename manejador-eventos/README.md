# Manejador de eventos
## Introducción
Proyecto final del curso de Heavy Duty Camp, Primeros pasos en Solana con Rust.

Consiste en elaborar un manejador de eventos descentralizados.
### Resumen
Manejador de eventos descentralizado. Permite:
- Crear eventos
- Participar como colaboradores
- Vender entradas
- Distribuir ganancias del evento
Los colaboradores obtendran tokens, de cierto evento escpecifico, por ejemplo EVENTO A. Una vez finalizado el evento se podrá obtener parte de los beneficios de dicho evento.
Es como un crowfunding.

Los tokens de cierto evento, tendran un precio 1:1 de la moneda que se vaya aceptar. Por ejemplo, si se acepta USDC para comprar las 'colaboraciones', tu token siempre tendra el valor de un USDC; si se hace con Solana, siempre tendra el valor de Solana.
Para ello:
- Existira una 'boveda del evento', donde se guardaran los ingresos de la venta de 'tokens de colaboración'.
- Existira una 'boveda de ganancias', donde se guardaran los ingresos de la venta de 'entradas al evento'.

Las ganancias se dividiran entre los colaboradores.
### Instrucciones
- Crear un evento
- Eliminar un evento
- Comprar tokens del evento (sponsor)
- Comprar entradas
- Retirar fondos del evento
- Finalizar un evento
- Retirar ganancias del evento
#### Crear un evento
##### Información
Nombre, identificador, descripcion, precio token evento, precio entradas
##### Cuentas PDA
- Token del evento
- Boveda del evento
- Boveda de ganancias

#### Eliminar un evento
Elimina un evento, al hacer esto estamos eliminando todas las cuentas asociadas, como el mint_account del token, las token_account de los usuarios, etc...
#### Finalizar un evento
Desactiva un evento, cambiando el atributo 'activo' de la struct `Evento`