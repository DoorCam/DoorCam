# Structure

## [Static](../static)

Contains all static files for the webserver,
e.g.: \*.css.

## [Templates](../templates)

All dynamic files for the webserver. The templating engine is [Tera](https://tera.netlify.app),
e.g.: \*.html.tera.
There is a [base file](../templates/base.html.tera) which nearly all other templates are derived from. For the admin views there is a superset of this called [admin_base](../templates/admin_base.html.tera).

## [Source](../src)

### [Template Contexts](../src/template_contexts)

Here are all contexts which are needed to generate the templates.

### [Requests](../src/requests)

All user-facing logic.

### [Database Entry](../src/db_entry)

All structs and functions used to communicate with the database and represent the data.

### [IoT](../../tree/scr/iot)

All logic which is not web-based.

#### [Door Control](../src/iot/door_control.rs)

Used to activate the door-opener.

#### [Bell Button](../src/iot/bell_button.rs)

Checks whether the button is pushed and sends a signal to the MQTT-Broker.

#### [Event Handler](../src/iot/event_handler.rs)

Syncs the flats between web and iot and manages the BellButtons.

### [Guards](../src/guards.rs)

Are used for the authentification and authorization.
