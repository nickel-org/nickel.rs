{{> header }}
    <h1>
        Hello {{ name }}!
    </h1>
    {{#helper}}
        Hello {{ name }} from a helper function
    {{/helper}}
{{> footer }}
