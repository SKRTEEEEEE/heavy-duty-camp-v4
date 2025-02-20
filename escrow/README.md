# PDA Transfer ✏️ TO WRITE

## Introducción
PDA Transfer es un programa basado en Solana y Anchor que permite la creación y transferencia de tokens utilizando cuentas de dirección programada (PDA). El programa maneja la creación de una cuenta de almacenamiento de mint, la generación de tokens y su posterior transferencia con verificación de autoridad.

## Tecnologías y Dependencias
Este proyecto utiliza las siguientes tecnologías y dependencias:

- **Lenguaje**: Rust
- **Framework**: Anchor
- **Blockchain**: Solana
- **Dependencias principales**:
  - `anchor-lang`
  - `anchor-spl`
  - `solana-program`

## Instalación y Configuración
Para comenzar con el proyecto, sigue estos pasos:

### 1. Instalar Rust y Anchor
Debes tener Rust y Anchor instalados, si no los tienes, [puedes encontrar una pequeña guia para WSL aquí](../docs/todo.md#config), o buscar la [guía completa para la instalación de Solana y Anchor en la documentación oficial](https://solana.com/es/docs/intro/installation).


### 2. Clonar el Repositorio
```bash
git clone https://github.com/SKRTEEEEEE/heavy-duty-camp-v4.git
cd escrow
```

### 3. Compilar el Programa
```bash
anchor build
```
- Para poder compilar el programa, deberemos configurar anchor en la version 0.29.0. Para ello, antes de crear el proyecto, deberemos utilizar el comando `avm use 0.29.0`. Una vez cambiado a la version, podremos iniciar el proyecto. 
Luego, **si estamos iniciando un nuevo proyecto**, en el archivo, `Cargo.toml`, añadir en dependencias, `anchor-spl: "0.29.0"`
- Para poder compilar el programa, deberemos probablemente adaptar el archivo `Cargo.lock` a la version requerida (probablemente 3).


### 4. Desplegar en Solana 
```bash
anchor deploy
```

### 5. Ejecutar Pruebas
**TENGO UN ERROR**
```bash
anchor test
```
##### ⬇️ TERMINAR
## Uso del Programa
El programa permite dos acciones principales:

1. **Mint de tokens**: Crear tokens en la blockchain.
2. **Transferencia de tokens**: Transferir tokens entre cuentas utilizando una PDA como autoridad.

Ejemplo de ejecución en Anchor CLI:

```bash
anchor run mint --supply 1000
anchor run transferir --quantity 100
```

## Agradecimientos
Gracias a la comunidad de Heavy Duty Builders, Solana y Anchor por la documentación y las herramientas que hacen posible el desarrollo de contratos inteligentes en Rust. 

También un agradecimiento especial a los contribuidores de este proyecto.
