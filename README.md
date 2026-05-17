# Simulación del Autómata Celular de Conway

Una implementación optimizada del Autómata Celular de John Conway (conocido popularmente como el Juego de la Vida), desarrollada de forma nativa en el lenguaje de programación Rust. El diseño se enfoca en la eficiencia computacional y el análisis cuantitativo de la dinámica poblacional, adaptándose a las directrices de rigor formal propias del ámbito académico de Modelos y Simulación.

---

## 1. Acceso y Descarga Directa

Para simplificar el proceso de evaluación y evitar la dependencia de entornos de compilación externos o herramientas de gestión de código por línea de comandos, el repositorio contiene el binario ejecutable precompilado para la arquitectura Windows. El acceso al paquete completo se realiza a través del siguiente enlace comprimido:

### [Descargar Paquete de Distribución (.ZIP)](https://github.com/TU_USUARIO/TU_REPOSITORIO/archive/refs/heads/main.zip)

*Nota: Sustituir la cadena de texto de la dirección URL por los identificadores reales del repositorio de destino tras el despliegue.*

---

## 2. Manual de Operación e Instrucciones de Ejecución

La ejecución de la aplicación no requiere la instalación previa del ecosistema de desarrollo de Rust ni de librerías de enlace dinámico de terceros, operando de manera autónoma y portátil.

### Procedimiento de Inicialización
1. Descargar el archivo comprimido utilizando el enlace del apartado anterior o mediante la interfaz gráfica de la plataforma (botón `Code` seguido de la opción `Download ZIP`).
2. Extraer el contenido del archivo comprimido en un directorio local del almacenamiento secundario del sistema de cómputo.
3. Acceder a la carpeta raíz resultante e iniciar la simulación mediante un doble clic sobre el script de procesamiento por lotes denominado `Iniciar_Simulacion.bat`.

### Control del Entorno de Simulación
* **Procesamiento Secuencial:** La aplicación actualiza los estados de forma periódica con un retardo controlado por hardware de 100 milisegundos por transición, garantizando un flujo estable aproximado de 10 iteraciones por segundo.
* **Finalización del Proceso:** Para interrumpir la simulación y suspender el bucle infinito del programa de forma segura, el usuario debe pulsar la combinación de teclado `Ctrl + C` en la ventana activa de la terminal o realizar el cierre directo de la interfaz de consola.

---

## 3. Tecnologías y Estructura del Proyecto

La arquitectura del sistema ha sido estructurada de forma modular, segregando el código fuente ejecutable de los archivos de configuración y los scripts de automatización del usuario final:

* `Cargo.toml`: Archivo de especificación del proyecto, gobernado por la Edición 2024 de Rust y el gestor Cargo. Define el uso de la biblioteca `rand` (versión 0.8.6) para el establecimiento de la entropía de la matriz.
* `src/main.rs`: Código fuente del sistema que agrupa el punto de entrada (`main`), el subsistema de renderizado por caracteres y el motor de cálculo matemático.
* `the_game_of_life.exe`: Compilado binario nativo optimizado con la bandera de producción del compilador (`--release`).
* `Iniciar_Simulacion.bat`: Script en lenguaje de comandos de Windows encargado de instanciar la consola de manera persistente, previniendo el cierre abrupto del buffer de salida al finalizar el proceso.

---

## 4. Estrategias de Optimización de Bajo Nivel

A diferencia de los enfoques convencionales fundamentados en arreglos bidimensionales indexados por punteros encadenados, este diseño integra tres técnicas de optimización orientadas al aprovechamiento del hardware de la CPU:

### Aplanamiento de la Matriz (Vector Unidimensional)
El plano celular de dimensiones $50 \times 50$ se almacena de forma contigua en un único vector indexado en memoria dinámica (`Vec<u8>`). La asignación espacial se determina mediante la función lineal de transformación de coordenadas $I(x, y) = (y \times L_p) + x$, donde $L_p$ representa el ancho expandido. Esto elimina la fragmentación de la memoria y maximiza la localidad de referencia, asegurando una tasa óptima de aciertos en las líneas de la memoria caché del procesador.

### Técnica de Elementos Centinela (Padding Perimetral)
Para mitigar el coste computacional asociado a la evaluación constante de bifurcaciones condicionales (operaciones de decisión `if`) en las fronteras de la matriz, el espacio vectorial se expande artificialmente mediante un marco perimetral inactivo, resultando en un volumen real de $52 \times 52$ unidades. El algoritmo restringe la iteración matemática estrictamente a las celdas internas, posibilitando el cálculo de la suma aritmética de las ocho vecindades mediante desplazamientos constantes y lineales sobre el vector, eludiendo fallos por desbordamiento de buffer (*Buffer Overflow*).

### Intercambio Instantáneo de Buffers de Memoria (Double Buffering vía std::mem::swap)
El sistema asigna dos espacios de memoria paralelos denominados `grid_a` (matriz de evaluación estática) y `grid_b` (lienzo de escritura de estados futuros). Una vez concluida la transición generacional, en lugar de replicar secuencialmente los valores de un vector a otro, la aplicación invoca el método nativo `std::mem::swap`. Esta función reasigna exclusivamente las referencias de las variables en la pila (*stack*) con un coste de complejidad constante $O(1)$, suprimiendo por completo la transferencia masiva de bytes en el bloque de memoria principal (*heap*).

---

## 5. Monitoreo Demográfico en Tiempo Real (Métricas del Modelo)

Con el propósito de proveer un valor analítico y formal a la simulación, el motor lógico efectúa un escrutinio exhaustivo simultáneo a la revaluación de estados. En cada ciclo transicional se registran y computan las siguientes variables agregadas en la interfaz visual de la terminal:

* **Población Absoluta:** Recuento cuantificado del volumen total de unidades biológicas activas que coexisten en el sistema.
* **Tasa de Natalidad (Nacimientos):** Cuantificación de transiciones positivas del estado inactivo ($0$) al estado activo ($1$) motivadas por la regla de densidad crítica.
* **Tasa de Mortalidad (Muertes):** Registro acumulado de ceses biológicos inducidos por escenarios de aislamiento demográfico o sobrepoblación en el entorno.