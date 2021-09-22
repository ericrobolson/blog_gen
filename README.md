An experiment in hexagonal architecture, type driven development and compiler design.

Roadmap
- [x] Parse env args for input directory + output directory
- [x] Crawl various file types, determining the types, and reject anything not whitelisted.
- [X] Spit out generic html file
- [x] Programmatically generate CSS
- [x] Add 'padding: 16px' to content for blog stuff. `box-sizing` doesn't respect margins.
- [x] Parsing IR (start with codeblock)
- [x] Bullets
- [x] Generating output
- [x] Add a margin top for content; e.g. 16px
- [x] Make headers start cards
- [x] Nested bullets (ideas2.html)
- [x] Escape HTML (css2.html), https://crates.io/crates/html-escape
- [x] Add image links
- [x] Add video links
- [x] Add iframes
- [x] Add external pages
- [x] Inline code quotes
- [x] Add fragments to headers
- [x] Add metadata parsing + requirements/warnings
- [x] Enforce summary length and keywords length; look at SEO
- [x] Enforce paragraphs after 3-5 sentences or 144-256 characters. Add in paragraph tags as well.
- [x] Fix quotes on 2021.08.07
- [x] Treeshaking
- [x] Delete styles.css
- [x] Validate links + images
- [x] Removed. Add page links (md)?
- [x] Lowercase all filenames + links
- [x] Analyzing IR; checking links, images, file names, etc.
- [x] Navigation
- [x] Restructure blog to utilize folders as well.
- [x] Add a 'series' tag like how you parse metadata. Perhaps you should update the way you parse metadata? Not sure if it's needed at the block level.
- [x] Add a `config.json` file which will populate `styles.css`. Ensure this is done at the context level. Add a 'social media' section which will link your twitter, linked in, etc.
- [x] Add a 'About Me' page which links social media + a blurb in the config.json.
- [x] Add a 'projects' page
- [x] Add a 'Catalog' page with links to all posts, grouped by month and ordered by date
- [x] Move images to subfolders? 
- [x] Add a top nav that links to about me, index, project, catalog, and last post to all pages. 
- [x] Index page containing series? Also contains last post?
- [ ] Post `blog_gen` into a separate repo.
- [ ] Move `blog` to a separate repo. 

V2 GOALS
- [ ] Move series to separate page
- [ ] Make content in a div, with overflow auto?
- [ ] Side nav for non-mobile?
- [ ] Perhaps add a JS search method? This will read from a JSON object that gets cached?
- [ ] Perhaps return success or error? Add a 'post deploy' script section, which will execute after running this? That way you can add in deployment as a final step here.
- [ ] STRETCH Fractal generator based on keywords and the like
- [ ] STRETCH: Add some new types, a 'UnescapedText' type and a 'HtmlEscapedText' type. Then you can ensure that only escaped text gets passed to HTML generation.
- [ ] STRETCH: Add warnings + errors to lists and blocks