{% if metadata.path %}
The following snippet is part of the file `{{ metadata.path }}`
{% endif %}

## Metadata associated with this snippet

{% for key, value in metadata -%}
{% if key == 'path' or not value -%}{% continue -%}{% endif -%}
**{{ key }}**:
{{ value }}
{% endfor -%}

## Content

````
{{ content }}
````
