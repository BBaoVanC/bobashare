searchState.loadedDescShard("syntect", 0, "Welcome to the syntect docs.\nA path given to a method was invalid. Possibly because it …\nAn error enum for all things that can go wrong within …\nFormatting error\nIO Error\nerror reading a file\nCommon error type used by syntax and theme loading\nAn error occurred while loading a syntax or theme\na syntax file was invalid in some way\na theme file was invalid in some way\nAn error occurred while parsing\na theme’s Plist syntax was invalid in some way\nScope error\nerror finding all the files in a directory\nMethods for dumping serializable structs to a compressed …\nAPI wrappers for common use cases like highlighting …\nReturns the argument unchanged.\nReturns the argument unchanged.\nEverything having to do with turning parsed text into …\nRendering highlighted code as HTML+CSS\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nEverything about parsing text into text annotated with …\nConvenient helper functions for common use cases:\nDumps an object to a binary array in the same format as …\nDumps an encodable object to a file at a given path, in …\nTo be used when serializing a <code>SyntaxSet</code> to a file. A …\nDumps an object to the given writer in a compressed binary …\nReturns a fully loaded object from a binary dump.\nReturns a fully loaded object from a binary dump file.\nA helper function for decoding and decompressing data from …\nTo be used when deserializing a <code>SyntaxSet</code> from raw data, …\nTo be used when deserializing a <code>SyntaxSet</code> that was …\nConvenience struct containing everything you need to …\nSimple way to go directly from lines of text to colored …\nIterator over the ranges of a line which a given the …\nA convenience wrapper over <code>ScopeRangeIterator</code> to return …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThis starts again from a previous state, useful for …\nHighlights a line of a file\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructs a file reader and a line highlighter to get you …\nReturns the current highlight and parse states, useful for …\nThe color black (<code>#000000</code>)\nBold font style\nRGBA color, directly from the theme\nThe color-independent styling of a font - i.e. bold, …\nHighlights a line of parsed code given a <code>HighlightState</code> …\nKeeps a stack of scopes and styles as state between …\nBasically a wrapper around a <code>Theme</code> preparing it to be used …\nItalic font style\nIncorrect Plist syntax\nHighlights a line of parsed code given a <code>HighlightState</code> …\nA single selector consisting of a stack to match and a …\nA selector set that matches anything matched by any of its …\nAn error parsing a settings file\nForeground and background colors, with font style\nA change to a <code>Style</code> applied incrementally by a theme rule\nA theme parsed from a <code>.tmTheme</code> file.\nA component of a theme meant to highlight a specific thing …\nProperties for styling the UI of a text editor\nUnderline font style\nThe color white (<code>#FFFFFF</code>)\nAlpha (transparency) component\nA color made available for use by the theme.\nColor of the guide lined up with the caret. Only applied …\nLoad all the themes in the folder into this <code>ThemeSet</code>\nReturns the set containing all flags.\nApplies a change to this style, yielding a new changed …\nApplies the other modifier to this one, creating a new …\nBlue component\nBackground color\nBackground color\nThe default backgound color of the view.\nReturns the intersection between the two sets of flags.\nDisables all flags disabled in the set.\nReturns the union of the two sets of flags.\nAdds the set of flags.\nReturns the raw value of the flags currently stored.\nReturns the left flags, but with all the right flags …\nToggles the set of flags.\nColor of bracketed sections of text when the caret is in a …\nControls certain options when the caret is in a bracket …\nBackground color of the brackets when the caret is next to …\nForeground color of the brackets when the caret is next to …\nControls certain options when the caret is next to a …\nColor of the caret.\nReturns the complement of this set of flags.\nReturns <code>true</code> if all of the flags in <code>other</code> are contained …\nReturns the difference between the flags in <code>self</code> and <code>other</code>.\nReturns all the themes found in a folder\nChecks if this selector matches a given scope stack.\nChecks if any of the given selectors match the given scope …\nReturns an empty set of flags.\nExtract all selectors for generating CSS\nIf this selector is really just a single scope, return it\nBackground color of regions matching the current search.\nText color of regions matching the current search.\nStyle of the font\nStyle of the font\nForeground color\nForeground color\nThe default color for text.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert from underlying bit representation, unless that …\nConvert from underlying bit representation, dropping any …\nConvert from underlying bit representation, preserving all …\nParses a scope stack followed optionally by (one or more) …\nParses a series of selectors separated by commas or pipes\nGreen component\nThe default style in the absence of any matched rules. …\nLoads a theme given a path to a .tmTheme file\nColor of the guides displayed to indicate nesting levels.\nBackground color of the gutter.\nForeground color of the gutter.\nThe border color for “other” matches.\nThe background color of a selection in a view that is not …\nA color that will override the scope-based text color of …\nInserts the specified flags in-place.\nReturns the intersection between the flags in <code>self</code> and …\nReturns <code>true</code> if there are flags common to both <code>self</code> and …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns <code>true</code> if all flags are currently set.\nReturns <code>true</code> if no flags are currently stored.\nColor of the line the caret is in. Only used when the …\nLoads the set of default themes Currently includes (these …\nGenerate a <code>ThemeSet</code> from all themes in a folder\nLoads a theme given a readable stream\nThe color of the border drawn around the viewport area of …\nThe color to use for the squiggly underline drawn under …\nNote that the <code>Highlighter</code> is not stored; it is used to …\nCreates an empty set\nYields the next token of text and the associated <code>Style</code> to …\nYields the next token of text and the associated <code>Style</code> to …\nReturns the complement of this set of flags.\nCSS passed to phantoms.\nCSS passed to popups.\nRed component\nRemoves the specified flags in-place.\nTarget scope name.\nThe styling rules for the viewed text\nThe background color of selected text.\nColor of the selection regions border.\nA color that will override the scope-based text color of …\nThe selectors, if any of them match, that this matches\nInserts or removes the specified flags depending on the …\nExternal settings for the editor using this theme\nThe color of the shadow used when a text area can be …\nColor of the current guide’s parent guide level. Only …\nThe style to use for this component\nReturns the fully resolved style for the given stack.\nReturns a <code>StyleModifier</code> which, if applied to the default …\nReturns the set difference of the two sets of flags.\nDisables all flags enabled in the set.\nReturns the symmetric difference between the flags in <code>self</code> …\nColor of tags when the caret is next to a tag. Only used …\nControls certain options when the caret is next to a tag. …\nToggles the specified flags in-place.\nReturns the union of between the flags in <code>self</code> and <code>other</code>.\nOutput HTML for a line of code with <code>&lt;span&gt;</code> elements using …\nOnly set the <code>background-color</code> if it is different than the …\nDetermines how background color attributes are generated\nDon’t include <code>background-color</code>, for performance or so …\nThe classes are the atoms of the scope separated by spaces …\nLike <code>Spaced</code>, but the given prefix will be prepended to all …\nSet background color attributes on every node\nLike <code>styled_line_to_highlighted_html</code> but appends to a …\nCreate a complete CSS for a given theme. Can be used …\nClose all open <code>&lt;span&gt;</code> tags and return the finished HTML …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvenience method that combines …\nConvenience method that combines …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nOutput HTML for a line of code with <code>&lt;span&gt;</code> elements …\nParse the line of code and update the internal HTML buffer …\nParse the line of code and update the internal HTML buffer …\nReturns a <code>&lt;pre style=&quot;...&quot;&gt;\\n</code> tag with the correct …\nOutput HTML for a line of code with <code>&lt;span&gt;</code> elements using …\nPreserved for compatibility, always use …\nMultiplier on the power of 2 for MatchPower. This is only …\nA reference to another file that is invalid\nUsed for <code>ScopeStack::apply_with_hook</code>\nUsed for the <code>clear_scopes</code> feature\nThe file must contain at least one YAML document\nA scope that syntect’s scope implementation can’t …\nInvalid YAML file syntax, or at least something yaml_rust …\nSyntaxes must have a context named “main”\nWrapper to get around the fact Rust <code>f64</code> doesn’t …\nA context is missing. Usually caused by a syntax …\nSome keys are required for something to be a valid …\nNot all strings are valid scopes\nKeeps the current parser state (the internal syntax …\nErrors that can occur while parsing.\nAn abstraction for regex patterns.\nInvalid regex\nA region contains text positions for capture groups in a …\nRestores cleared scopes\nThe global scope repo, exposed in case you want to …\nA hierarchy of atoms with semi-standardized names used to …\nScope related errors\nThe structure used to keep track of the mapping between …\nA stack/sequence of scopes for representing hierarchies …\nA change to a scope stack\nA linked version of a <code>SyntaxDefinition</code> that is only useful …\nA syntax set holds multiple syntaxes that have been linked …\nA syntax set builder is used for loading syntax …\nDue to a limitation of the current optimized internal …\nThe internal representation uses 16 bits per atom, so if …\nSome part of the YAML file is the wrong type (e.g a string …\nAdd a syntax to the set.\nLoads all the <code>.sublime-syntax</code> files in a folder into this …\nA rarely useful method that loads in a syntax with no …\nModifies this stack according to the operation given\nModifies this stack according to the operation given and …\nReturn a slice of the scopes in this stack\nGets the atom number at a given index.\nReturn the string for an atom number returned by …\nReturns the bottom <code>n</code> elements of the stack.\nBuild a <code>SyntaxSet</code> from the syntaxes that have been added …\nReturns a string representation of this scope\nPrints out each scope in the stack separated by spaces and …\nChecks if this stack as a selector matches the given …\nTry to find the syntax for a file based on its first line\nSearches for a syntax by it’s original file path when it …\nFinds a syntax by its default scope, for example …\nSearches for a syntax first by extension and then by …\nConvenience method that tries to find the syntax for a …\nFinds a syntax for plain text, which usually has no …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nParses a scope stack from a whitespace separated list of …\nNote: creating a ScopeStack with this doesn’t contain …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts this syntax set into a builder so that more …\nCheck if the regex matches the given text.\nTests if this scope is a prefix of another scope. Note …\nReturns the number of atoms in the scope\nReturn the height/length of this stack\nSame as <code>load_defaults_nonewlines</code> but for parsing line …\nInstantiates a new syntax set from a binary dump of …\nConvenience constructor for creating a builder, then …\nCreates a state from a syntax definition, keeping its own …\nCreate a new regex from the pattern string.\nParses a <code>Scope</code> from a series of atoms separated by dot (<code>.</code>) …\nParses a single line of the file. Because of the way regex …\nGet the start/end positions of the capture group with …\nReturn the regex pattern.\nSearch for the pattern in the given text from begin/end …\nData structures for representing syntax definitions\nThe list of syntaxes in the set\nThe list of syntaxes added so far.\nCheck whether the pattern compiles as a valid regex or not.\nAn opaque ID for a <code>Context</code>.\nUsed to iterate over all the match patterns in a context\nThe main data structure representing a syntax definition …\nReturns an iterator over all the match patterns in this …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nget the context ID this reference points to\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIn case you want to create your own SyntaxDefinition’s …\nReturns the match pattern at an index\nThis being set false in the syntax file implies this field …\nThis is filled in by the linker at link time for contexts …\nUsed by the parser to compile a regex which needs to …\nfind the pointed to context\n<code>true</code> if this reference by scope is part of an <code>embed</code> for …\nSame semantics as for <code>Self::ByScope::with_escape</code>.\nAn iterator over the lines of a string, including the line …\nFormats the styled fragments using 24-bit color terminal …\nFormats the styled fragments using LaTeX textcolor …\nPrint out the various push and pop operations in a vector …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nModify part of a highlighted line using a style modifier, …\nSplit a highlighted line at a byte index in the line into …")