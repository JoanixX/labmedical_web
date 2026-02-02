# LabMedical - Plataforma de Catalogo Medico B2B

Plataforma profesional B2B de catalogo para LabMedical, proveedor de productos medicos. Esta plataforma sirve como catalogo digital con sistema de cotizaciones para clientes empresariales.

## Estructura del Proyecto

```
labmedical_web/
├── backend/          # API Rust/Axum
├── frontend/         # Aplicacion web Astro/TypeScript (proximamente)
├── docs/            # Documentacion
└── README.md
```

## Stack Tecnologico

### Backend

- **Lenguaje**: Rust
- **Framework**: Axum 0.7
- **Base de Datos**: PostgreSQL
- **Autenticacion**: JWT + bcrypt
- **Almacenamiento**: AWS S3
- **Email**: API Resend
- **Hosting**: Render.com

### Frontend (Proximamente)

- **Framework**: Astro 4.x
- **Lenguaje**: TypeScript
- **Estilos**: TailwindCSS
- **Hosting**: Vercel

## Funcionalidades

### Funcionalidades Publicas

- Catalogo de productos con busqueda y filtros
- Paginas de detalle de productos
- Navegacion por categorias
- Sistema de solicitud de cotizaciones
- Formulario de contacto
- Paginas de informacion de la empresa

### Funcionalidades Administrativas

- Autenticacion segura
- Gestion de productos (CRUD)
- Gestion de categorias
- Bandeja de entrada y gestion de cotizaciones
- Carga de imagenes a S3
- Panel de control con metricas

## Modelo de Negocio

Esta es una **plataforma B2B**, no un sitio de comercio electronico:

- Sin pagos directos ni carrito de compras
- Los clientes navegan el catalogo y solicitan cotizaciones
- El equipo de ventas maneja las negociaciones offline
- Enfoque en presentacion profesional y generacion de leads

## Comenzando

### Configuracion del Backend

Ver [backend/README.md](./backend/README.md) para instrucciones detalladas.

Inicio rapido:

```bash
cd backend
cp .env.example .env
# configurar .env con tus credenciales
cargo run
```

### Configuracion del Frontend

Proximamente...

## Documentacion

- [Documentacion de API](./docs/API.md)
- [Guia de Deployment](./docs/DEPLOYMENT.md)
- [Configuracion de Desarrollo](./docs/SETUP.md)

## Acceso de Administrador por Defecto

- Email: `admin@labmedical.com`
- Contraseña: `admin123`

**⚠️ Cambiar inmediatamente en produccion**

## Deployment

### Backend (Render.com)

- Tier gratuito con PostgreSQL incluido
- Deployment automatico desde GitHub
- Ver `backend/render.yaml` para configuracion

### Frontend (Vercel)

- Tier gratuito con CDN global
- Deployment automatico desde GitHub
- Configuracion proximamente

## Caracteristicas de Seguridad

- Autenticacion JWT con expiracion de 24h
- Hashing de contraseñas con bcrypt
- Validacion de entrada (cliente + servidor)
- Rate limiting en endpoints publicos
- Proteccion contra inyeccion SQL
- Configuracion CORS
- Validacion de carga de archivos

## Esquema de Base de Datos

- **products**: Catalogo de productos con especificaciones JSONB
- **categories**: Categorizacion de productos
- **quotes**: Solicitudes de cotizacion de clientes
- **admins**: Usuarios administradores con contraseñas hasheadas

## Endpoints de API

### Publicos

- `GET /api/products` - Listar productos
- `GET /api/products/:slug` - Detalles de producto
- `GET /api/categories` - Listar categorias
- `POST /api/quotes` - Enviar solicitud de cotizacion

### Administrador (requiere JWT)

- `POST /api/admin/login` - Autenticacion
- CRUD completo para productos, categorias y cotizaciones
- `POST /api/admin/upload` - Carga de imagenes

## Licencia

Propietario - LabMedical

## Contacto

Para preguntas o soporte, contactar: ventas@labmedical.com

---

Construido con ❤️ para LabMedical
