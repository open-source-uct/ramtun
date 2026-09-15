# Ramtun. Descripción del sistema

## 1. Resumen

Ramtun es una plataforma institucional para tomar evaluaciones en línea dentro de una universidad. Los académicos crean cursos, arman bancos de preguntas y montan cuestionarios sincronizados que los estudiantes rinden en el navegador con restricciones que protegen la integridad académica. El sistema corrige cada evaluación automáticamente, entrega una nota en escala chilena y publica los resultados a los estudiantes.

Su propósito es reemplazar las evaluaciones presenciales en papel por una evaluación digital controlada: la corrección es automática y las conductas irregulares quedan registradas.

## 2. Perfiles de usuario

- Administrador del sistema: ve todos los cursos de la plataforma, gestiona usuarios y define sus roles.
- Académico (docente): crea y administra sus cursos, arma bancos de preguntas, crea y publica cuestionarios, y supervisa las evaluaciones en curso.
- Ayudante (docente de curso): participa de un curso específico y colabora en la supervisión de sus evaluaciones.
- Estudiante: se une a un cuestionario mediante su código de acceso, lo rinde en el navegador y consulta sus resultados al publicarse.

Los roles generales (Administrador, Académico, Estudiante) definen qué puede hacer cada persona. Dentro de un curso, los participantes se organizan además como Académico o Ayudante, y esa distinción determina quién supervisa las evaluaciones.

El acceso de una persona a un curso se otorga al incorporarla como participante; sin pertenencia al curso no puede ver su contenido ni rendir sus evaluaciones.

## 3. Autenticación y sesión

- El usuario accede a la plataforma con las credenciales institucionales de su cuenta universitaria (usuario y contraseña).
- Al autenticarse correctamente, el sistema abre una sesión y dirige al usuario según su perfil.
- La sesión permanece activa a lo largo de la navegación y se restaura automáticamente al volver a la plataforma.
- El usuario cierra la sesión de forma explícita desde el encabezado; al hacerlo, queda fuera de la plataforma.
- El sistema recuerda la última fecha de acceso de cada usuario.
- La barra superior presenta al usuario por su nombre y su perfil mientras navega.

## 4. Gestión de cursos

- El usuario con permiso (Administrador o Académico) crea un curso indicando nombre, código y año académico.
- El usuario ve el listado de cursos a los que pertenece; el Administrador ve todos los cursos del sistema.
- Abrir un curso muestra sus datos generales, sus participantes, sus bancos de preguntas y sus cuestionarios.
- Un curso puede eliminarse; queda fuera de la plataforma.
- El responsable de un curso incorpora participantes (académicos, ayudantes y estudiantes) y también puede quitarlos.
- Solo quienes administran el curso incorporan o retiran participantes; los estudiantes solo ven los cursos a los que pertenecen.

## 5. Bancos de preguntas

- El usuario puede crear un banco de preguntas dentro de un curso, con nombre propio.
- Un banco contiene un conjunto de preguntas de opción múltiple con una única respuesta correcta.
- Cada pregunta se define por su enunciado, sus opciones y la indicación de cuál es la correcta.
- El nombre del banco y el texto de las preguntas y opciones pueden editarse.
- La edición no permite cambiar la respuesta correcta señalada en cada pregunta, ni agregar, quitar o reordenar preguntas u opciones.
- Un banco puede eliminarse; deja de estar disponible.
- Cuando un banco está en uso por un cuestionario con evaluaciones en curso o resultados comprometidos, el sistema bloquea su modificación hasta que el cuestionario deje de depender de él.
- El estudiante con acceso al curso puede ver el contenido de los bancos para estudiar, pero el sistema nunca le muestra cuál es la respuesta correcta.
- Las preguntas que un cuestionario toma de un banco quedan asociadas a ese cuestionario: cambios posteriores al banco no alteran lo que los estudiantes ven ni lo que el sistema corrige.

## 6. Cuestionarios

- El usuario crea un cuestionario dentro de un curso, indicando título, modalidad, fecha de inicio, duración de cada rendición y cantidad de preguntas.
- El cuestionario se arma seleccionando uno o más bancos de preguntas del curso; las preguntas se toman de ellos.
- Existen dos modalidades de evaluación:
  - Tradicional: cada pregunta vale igual; se acierta o se falla.
  - Por certeza: el estudiante indica su nivel de seguridad al responder (bajo, medio, alto); a mayor certeza, mayor puntaje si acierta, pero mayor descuento si falla.
- La configuración de la modalidad por certeza define los puntos otorgados y descontados en cada nivel de certeza.
- La cantidad de preguntas del cuestionario no puede superar la cantidad disponible en los bancos seleccionados.
- Al crearse, el sistema asigna un código de acceso de ocho caracteres que el académico comparte con los estudiantes para que se unan.
- Un cuestionario no puede iniciarse antes de su fecha de inicio.
- El académico puede eliminar un cuestionario mientras sea seguro hacerlo (sin resultados comprometidos).
- Cuando el académico cierra el cuestionario, el sistema publica masivamente los resultados de todas las evaluaciones rendidas.
- Un cuestionario ya publicado no acepta nuevas rendiciones ni puede volver a publicarse.

## 7. Rendición de evaluaciones

- El estudiante ingresa el código de acceso del cuestionario y el sistema le muestra una vista previa: título, modalidad, cantidad de preguntas, duración y horario de inicio.
- No es posible unirse a un cuestionario cuyo horario aún no comienza o cuyos resultados ya fueron publicados.
- Al iniciar la rendición, el sistema entrega al estudiante una cantidad de preguntas seleccionadas al azar del cuestionario, en un orden distinto para cada estudiante.
- La rendición tiene una duración límite establecida por el académico; al vencer el tiempo, el sistema deja de aceptar respuestas (con una breve tolerancia de gracia).
- El estudiante ve el tiempo restante durante toda la rendición.
- Mientras el tiempo esté vigente, el estudiante puede responder, cambiar sus respuestas y navegar entre preguntas; cada respuesta queda guardada.
- En la modalidad por certeza, el estudiante debe indicar su nivel de certeza en cada respuesta; en la modalidad tradicional, el sistema no solicita ni acepta ese dato.
- El estudiante envía su evaluación de forma explícita; a partir de ese momento ya no puede modificarla.
- Un estudiante solo puede rendir un cuestionario una vez; si ya envió una evaluación, no puede comenzar otra para el mismo cuestionario.
- Si el estudiante abandona la plataforma con una rendición en curso, el sistema lo devuelve automáticamente a esa rendición al volver.

## 8. Calificación

- El sistema corrige cada evaluación en el momento en que se envía.
- En la modalidad tradicional, cada respuesta correcta suma un punto y cada incorrecta no suma puntos.
- En la modalidad por certeza, los puntos dependen del nivel de certeza indicado:
  - Certeza baja: acierta suma 1 punto; falla suma 0.
  - Certeza media: acierta suma 2 puntos; falla descuenta 2 puntos.
  - Certeza alta: acierta suma 3 puntos; falla descuenta 4 puntos.
- Las preguntas sin responder no suman puntos y cuentan como incorrectas.
- El puntaje final se convierte a una nota de escala 1.0 a 7.0, en la que el puntaje máximo equivale a 7.0.
- Si el puntaje final de un estudiante es igual o menor a cero, la nota queda en 1.0.
- El puntaje máximo posible de un cuestionario queda definido por su modalidad y su cantidad de preguntas.

## 9. Resultados

- Mientras el cuestionario no ha sido cerrado, el estudiante no puede ver sus resultados.
- Al cerrar y publicar el cuestionario, el sistema habilita los resultados a todos los estudiantes que lo rindieron.
- El estudiante consulta su resultado mediante el código de acceso del cuestionario.
- El resultado muestra la nota obtenida, el puntaje logrado, el puntaje máximo y el detalle por pregunta: qué respondió, cuál era la correcta, su nivel de certeza (si correspondía) y los puntos obtenidos en cada una.
- El estudiante puede consultar su resultado una sola vez; una vez visto, el sistema no vuelve a mostrarlo.
- El académico y el ayudante del curso pueden ver el resultado individual de cualquier estudiante con todo el detalle, sin la restricción de la consulta única del estudiante.
- El responsable del curso ve el listado de rendiciones de un cuestionario: quién rindió, cuándo comenzó, cuándo envió, si vio sus resultados y su nota.

## 10. Integridad académica (modo supervisado)

- Durante la rendición, el sistema bloquea acciones que comúnmente se usan para hacer trampa: abrir el menú contextual, copiar texto, buscar dentro de la página, capturar pantalla y cambiar de ventana o aplicación.
- Si el usuario intenta copiar contenido de la evaluación, el sistema reemplaza lo copiado por un aviso que desaconseja procesarlo, para neutralizar la ayuda de asistentes digitales.
- Cada acción irregular detectada se registra como un incidente asociado al estudiante y a su evaluación, con tipo, descripción y número secuencial.
- Los incidentes repetidos dentro de un período muy corto se agrupan para evitar duplicados masivos.
- El responsable del curso ve la lista de incidentes registrados, tanto por evaluación como por estudiante.

## 11. Avisos en tiempo real

- El sistema notifica al responsable del curso en tiempo real cada vez que un estudiante inicia una rendición y cada vez que uno envía su evaluación.
- Cada incidente de conducta irregular se comunica también en tiempo real a los responsables del curso.
- Estas notificaciones permiten al académico detectar conductas sospechosas mientras la evaluación está en curso.

## 12. Reglas transversales

- El sistema conserva el historial: al eliminar cursos, bancos, preguntas, cuestionarios o rendiciones, el registro queda guardado y deja de estar visible, pero la información de evaluaciones ya rendidas se preserva íntegra.
- Las preguntas que forman parte de una evaluación rendida mantienen su contenido original incluso si el banco de preguntas fue modificado o eliminado posteriormente.
- Los resultados obtenidos y publicados no pueden ser alterados.
- La información confidencial de cada evaluación (respuesta correcta de cada pregunta) nunca se entrega al estudiante antes de la publicación, y la identidad de la respuesta correcta permanece oculta incluso al estudiar los bancos.
- Un mismo estudiante no puede tener más de una rendición activa a la vez en la plataforma.
