# Structure

## [Static](../static)

Contains all static files for the webserver,
e.g.: \*.css.

## [Templates](../templates)

All dynamic files for the webserver. The templating engine is [Tera](https://tera.netlify.app),
e.g.: \*.html.tera.
There is a [base file](../templates/base.html.tera) which nearly all other templates are derived from. For the admin views there is a superset of this called [admin_base](../templates/admin_base.html.tera).

## [Source](../src)

This folder contains the Rust source code and unit-tests,
e.g.: \*.rs, \*\_test.rs.

### [Template Contexts](../src/template_contexts)

Here are all contexts which are needed to generate the HTML from the templates.

### [Requests](../src/requests)

All user-facing logic.

### [Database Entry](../src/db_entry)

All structs and functions used to communicate with the database and represent the data.

### [IoT](../src/iot)

All logic which is not web-based.

#### [Door Control](../src/iot/door_control.rs)

Used to activate the door-opener.

#### [Bell Button](../src/iot/bell_button.rs)

Checks whether the button is pushed and sends a signal to the MQTT-Broker.

#### [Event Handler](../src/iot/event_handler.rs)

Syncs the flats between web and IoT and manages the BellButtons.

### [Utils](../src/utils)

Small helper functions and structs which aren't apprppriate in other folders.

#### [Crypto](../src/utils/crypto.rs)

Cryptographic helper/wrapper function(s).

#### [Auth Manager](../src/utils/auth_manager.rs)

Is used for the authentification.

#### [Guards](../src/utils/guards.rs)

Are used for the authorization. See [Rocket documentation](https://rocket.rs/v0.4/guide/requests/#request-guards).
