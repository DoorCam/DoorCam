# Structure

## static

Contains all static files for the webserver.
e.g.: \*.css

## templates

All dynamic files for the webserver. The templating engine is [Tera](https://tera.netlify.app).
e.g.: \*.html.css
There is a [base file](./templates/base.html.tera) which nearly all other templates are derived from. For the admin views there is a superset of this called admin_base

## src

### template_contexts

Here are all contexts which are needed to generate the templates.

### requests

All user-facing logic.

### db_entry

All structs and functions used to communicate with the database and represent the data.

### iot

All logic which is not web-based.

#### door_control

Used to activate the door-opener.

#### bell_button

Checks whether the button is pushed and sends a signal to the MQTT-Broker.

#### event_handler

Syncs the flats between web and iot and manages the BellButtons.

### guards

Are used for the authentification and authorization.
