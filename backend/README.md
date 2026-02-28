# API LabMedical - Backend

API Backend para la plataforma de catalogo medico B2B LabMedical, construida con Rust y Axum.

## Stack Tecnologico

- **Framework**: Axum 0.7
- **Base de Datos**: PostgreSQL con SQLx
- **Autenticacion**: JWT con bcrypt
- **Almacenamiento de Archivos**: AWS S3
- **Email**: API Resend
- **Deployment**: Render.com

## Caracteristicas

- API RESTful con endpoints publicos y administrativos
- Autenticacion basada en JWT con bcrypt
- Catalogo de productos con busqueda y filtrado por categoria
- Sistema de solicitud de cotizaciones con notificaciones por email
- Carga de imagenes a AWS S3 con validacion de tipo y tamaño
- Operaciones CRUD para productos, categorias y cotizaciones
- Validacion de entrada con el crate `validator`
- Manejo centralizado de errores con codigos HTTP apropiados

## Arquitectura

```
backend/src/
├── main.rs          # punto de entrada, servidor y router
├── config.rs        # carga de variables de entorno
├── db.rs            # pool de conexiones a PostgreSQL
├── error.rs         # manejo centralizado de errores
├── models/          # structs de datos y DTOs
├── routes/          # handlers de endpoints
│   ├── public.rs    # endpoints publicos
│   └── admin.rs     # endpoints de administracion
├── services/        # logica de negocio
│   ├── auth.rs      # JWT y hashing de contraseñas
│   ├── email.rs     # notificaciones via Resend
│   └── s3.rs        # carga de imagenes a AWS S3
└── middleware/      # middleware de autenticacion
    └── auth.rs      # verificacion de JWT
```

## Comenzando

### Prerequisitos

- Rust 1.70+
- PostgreSQL 14+
- Bucket AWS S3
- Clave API de Resend

### Instalacion

1. Clonar el repositorio
2. Copiar `.env.example` a `.env` y configurar:

   ```bash
   cp .env.example .env
   ```

3. Configurar base de datos PostgreSQL:

   ```bash
   createdb labmedical
   ```

4. Ejecutar migraciones:

   ```bash
   cargo install sqlx-cli
   sqlx migrate run
   ```

5. Iniciar el servidor:
   ```bash
   cargo run
   ```

La API estara disponible en `http://localhost:3000`

## Endpoints de API

### Endpoints Publicos

- `GET /health` - Verificacion de salud
- `GET /api/products` - Listar productos (con paginacion, busqueda, filtros)
- `GET /api/products/:slug` - Obtener producto por slug
- `GET /api/categories` - Listar todas las categorias
- `POST /api/quotes` - Enviar solicitud de cotizacion

### Endpoints Administrativos (requieren JWT)

- `POST /api/admin/login` - Login de administrador
- `GET /api/admin/products` - Listar todos los productos (vista admin)
- `POST /api/admin/products` - Crear producto
- `PUT /api/admin/products/:id` - Actualizar producto
- `DELETE /api/admin/products/:id` - Eliminar producto
- `PATCH /api/admin/products/:id/toggle` - Alternar estado activo del producto
- `GET /api/admin/categories` - Listar categorias
- `POST /api/admin/categories` - Crear categoria
- `PUT /api/admin/categories/:id` - Actualizar categoria
- `DELETE /api/admin/categories/:id` - Eliminar categoria
- `GET /api/admin/quotes` - Listar cotizaciones
- `GET /api/admin/quotes/:id` - Obtener detalles de cotizacion
- `PATCH /api/admin/quotes/:id/status` - Actualizar estado de cotizacion
- `POST /api/admin/upload` - Subir imagen a S3

## Credenciales de Administrador por Defecto

- Email: `admin@labmedical.com`
- Contraseña: `admin123`

**⚠️ Cambiar estas credenciales inmediatamente en produccion**

## Variables de Entorno

Ver `.env.example` para todas las variables de entorno requeridas.

## Deployment

Este proyecto esta configurado para deployment en Render.com. Ver `render.yaml` para la configuracion.

## Licencia

Propietario - LabMedical
