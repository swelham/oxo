# oxi (need to think up a name)

## feature to do list

  - [ ] document validation
    - [x] must start with ```doctype``` or ```extends```
    - [ ] indentation should be consitant (this needs some thought about how strict this needs to be)
    - [ ] ```xml``` documents can only use basic tags and attributes
    - [ ] ```json``` validation considerations
  - [ ] fully implement support for all ```doctype``` options
     - [x] html
     - [x] xml
     - [ ] json
  - [x] basic tags
  - [x] nested tags
  - [x] self closing tags (this needs to take the ```doctype``` into account)
  - [x] classes (using ```.className``` syntax)
  - [x] attributes (using ```(attr="value")``` syntax)
  - [x] omittable div tag when using classes or attributes
  - [x] option to explictly self close a tag wit the ```tag/``` syntax
  - [x] plain text (using ```| some text``` syntax)
  - [x] pretty print
  - [x] html code blocks (applied when nested code is discovered under this tag - all nested code is not parsed)
     - [x] javascript (using standard ```script``` tag)
     - [x] css (using standard ```style``` tag)
  - [ ] code block minification when not using pretty print
  - [ ] filter blocks (using the ```:filter``` syntax)
     - [ ] ```:markdown```
     - [ ] ```:coffee``` (only in ```script``` code block)
     - [ ] ```:less``` (only in ```style``` code block)
     - [ ] ```:sass``` (only in ```style``` code block)
  - [x] code comments
     - [x] templates comments (ignored by parser)
     - [x] html/xml comments (renderd using standard xml comment syntax ```<!-- -->```)
     - [x] nested comments
  - [ ] mixins
  - [ ] inheritance (the ```extends``` feature)
     - [ ] blocks
     - [ ] nested blocks
     - [ ] block append
  - [ ] includes (allows injection of content from another document)
     - [ ] template inclusion (using ```include ./another_template.oxi```)
     - [ ] plain text inclusion (same syntax as above, all files without a template extension are treated as plain text)
     - [ ] include filtered file (using ```include:markdown ./file.md``` syntax)
  - [ ] data binding
  - [ ] sort this list to group features and add the remaining features
  - [ ] write documentation
  - [ ] testing
     - [ ] currently test are validating multiple features, break this down
     - [ ] unit tests
