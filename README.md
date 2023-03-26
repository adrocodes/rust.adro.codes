# rust.adro.codes

A simple markdown parser and site builder used for the rust.adro.codes website (pending progress in this repo).

Running main will do the following;

1. Collect all the `.md` & `.mdx` files under the `site` directory.
2. Set up the Liquid parser builder
3. Get the contents of the `templates/base.liquid` file.
4. Go through all the markdown files.
   1. Parse the contents of the file to HTML
   2. Ensure that the path in the `site` directory exists in the `public` directory. For example; `site/blog` will have a `public/blog` equivalent.
   3. Inject the parsed markup into the liquid template.
   4. Create the `.html` version in the `public` directory.

If you wanted custom CSS or JavaScript. You can add it to the `public` directory and link to it in the `base.liquid` template.