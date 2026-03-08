# Renta De Carros
🚗 Smart Contract: Inventario de Carros (Solana + Anchor)
Este proyecto es un contrato inteligente (programa) construido en la blockchain de Solana utilizando el framework Anchor. Permite a los usuarios crear, gestionar y administrar un inventario descentralizado de vehículos (CRUD completo).

⚙️ Características Principales
El programa cuenta con 5 instrucciones principales para interactuar con la blockchain:

crear_inventario: Inicializa una nueva cuenta PDA (Program Derived Address) vinculada al usuario (owner). Asigna un nombre al inventario y prepara el espacio para almacenar los carros.

<img width="2816" height="1536" alt="Gemini_Generated_Image_objkvoobjkvoobjk" src="https://github.com/user-attachments/assets/f0826421-ebf4-41e9-8f65-d74a6a6e0681" />


Agregar carro: Registra un nuevo vehículo en el inventario especificando su modelo y anio. Por defecto, el carro se marca como disponible.

<img width="1408" height="768" alt="image" src="https://github.com/user-attachments/assets/76ac38aa-6471-4c46-b430-c91ea4a22f9e" />


Ver carros: Imprime en la consola de la red (logs) la lista completa de carros almacenados en el inventario actual.

<img width="1408" height="768" alt="image" src="https://github.com/user-attachments/assets/c16892e6-4aa5-4a65-8d47-931d7744e378" />


Eliminar carro: Busca un carro por su modelo y lo elimina permanentemente del vector de almacenamiento.
Alternar estado: Cambia la disponibilidad de un carro (de true a false o viceversa), útil para marcar si un carro ha sido vendido o rentado.

<img width="1408" height="768" alt="image" src="https://github.com/user-attachments/assets/4a5a457d-db35-4028-ba99-fa81bff74a4b" />


🏗️ Estructuras de Datos
Inventario
Cuenta principal (PDA) que almacena la información general.

# *owner*: Llave pública del creador y administrador.

# *nombre*: Nombre del inventario (Máximo 60 bytes).

# *carros*: Un vector que contiene la lista de vehículos (Máximo 10 carros para optimizar el espacio).

# *Carro*: Estructura que define cada vehículo.

# *modelo*: Nombre o modelo del carro (String).

# *anio*: Año de fabricación (u16).

# *disponible*: Estado actual del vehículo (Booleano).

🚀 Guía de Despliegue (Solana Playground)
Si estás probando este contrato en Solana Playground (beta.solpg.io), sigue estos pasos para evitar errores de semillas (seeds) o de autoridad:

1. Configuración Inicial
Pega el código en tu archivo lib.rs.

Ve a la pestaña Build & Deploy (ícono de herramientas).

Copia el Program ID que te asigna el Playground.

Pega ese ID en la línea declare_id!("TU_PROGRAM_ID_AQUI"); de tu código.

2. Compilación y Despliegue
Abre la terminal integrada y asegúrate de tener saldo ejecutando: solana airdrop 2 (solo en Devnet/Testnet).

Presiona el botón Build (ícono de martillo) y espera el mensaje de éxito.

Presiona el botón Deploy.

3. Pruebas y Solución de Errores
Para probar las instrucciones en la pestaña Test (ícono de tubo de ensayo):

Importante: Si realizaste cambios en el código, recarga la página del navegador para limpiar el caché de la interfaz.

Abre la instrucción que deseas probar (ej. crear_inventario).

Presiona el botón Reset para limpiar direcciones antiguas.

Llena los argumentos requeridos (ej. nombre: "Lote de Autos").

Presiona el botón Fill para que el sistema calcule automáticamente la dirección PDA correcta usando las semillas ("inventario" + tu llave pública).

Presiona Test.

🛡️ Errores Personalizados
El contrato maneja excepciones para evitar fallos genéricos:

NoEresElOwner: Se dispara si alguien intenta modificar un inventario del cual no es propietario.

CarroNoExiste: Se dispara al intentar eliminar o cambiar el estado de un modelo que no se encuentra en el array.
