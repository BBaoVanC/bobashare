searchState.loadedDescShard("plist", 0, "Plist\nA byte buffer used for serialization to and from the plist …\nA UTC timestamp used for serialization to and from the …\nThis type represents all possible errors that can occur …\nAn integer that can be represented by either an <code>i64</code> or a …\nAn error indicating that a string was not a valid XML …\nA plist <code>uid</code> value. These are found exclusively in plists …\nRepresents any plist value.\nOptions for customizing serialization of XML plists.\nIf the <code>Value</code> is an Array, returns the associated <code>Vec</code>.\nIf the <code>Value</code> is an Array, returns the associated mutable …\nIf the <code>Value</code> is a Boolean, returns the associated <code>bool</code>.\nIf the <code>Value</code> is a Data, returns the associated <code>Vec</code>.\nIf the <code>Value</code> is a Date, returns the associated <code>Date</code>.\nIf the <code>Value</code> is a Dictionary, returns the associated …\nIf the <code>Value</code> is a Dictionary, returns the associated …\nReturns the underlying error if it was caused by a failure …\nIf the <code>Value</code> is a Real, returns the associated <code>f64</code>.\nReturns the value as an <code>i64</code> if it can be represented by …\nIf the <code>Value</code> is a signed Integer, returns the associated …\nIf the <code>Value</code> is a String, returns the associated <code>str</code>.\nIf the <code>Value</code> is a Uid, returns the associated <code>Uid</code>.\nReturns the value as a <code>u64</code> if it can be represented by …\nIf the <code>Value</code> is an unsigned Integer, returns the …\nA map of String to plist::Value.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nDeserializes an instance of type <code>T</code> from a byte slice.\nDeserializes an instance of type <code>T</code> from a plist file of …\nReads a <code>Value</code> from a plist file of any encoding.\nDeserializes an instance of type <code>T</code> from a seekable byte …\nReads a <code>Value</code> from a seekable byte stream containing a …\nDeserializes an instance of type <code>T</code> from a byte stream …\nReads a <code>Value</code> from a byte stream containing an ASCII …\nDeserializes an instance of type <code>T</code> from a byte stream …\nReads a <code>Value</code> from a byte stream containing an XML encoded …\nInterprets a <code>Value</code> as an instance of type <code>T</code>.\nCreate a <code>Data</code> object from an XML plist (Base-64) encoded …\nConverts an XML plist date string to a <code>Date</code>.\nReturns the value as a <code>u64</code>.\nSpecifies the character and amount used for indentation.\nSpecify the sequence of characters used for indentation.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIf the <code>Value</code> is a Array, returns the underlying <code>Vec</code>.\nIf the <code>Value</code> is a Data, returns the underlying <code>Vec</code>.\nIf the <code>Value</code> is a Dictionary, returns the associated …\nReturns the underlying error if it was caused by a failure …\nIf the <code>Value</code> is a String, returns the underlying <code>String</code>.\nIf the <code>Value</code> is a Uid, returns the underlying <code>Uid</code>.\nReturns true if this error was caused by prematurely …\nReturns true if this error was caused by a failure to read …\nCreates a new <code>Data</code> from vec of bytes.\nCreates a new <code>Uid</code> containing the given value.\nSelects whether to write the XML prologue, plist document …\nSerializes the given data structure to a file as a binary …\nSerializes a <code>Value</code> to a file as a binary encoded plist.\nSerializes the given data structure to a file as an XML …\nSerializes a <code>Value</code> to a file as an XML encoded plist.\nConverts a <code>T</code> into a <code>Value</code> which can represent any valid …\nSerializes the given data structure to a byte stream as a …\nSerializes a <code>Value</code> to a byte stream as a binary encoded …\nSerializes the given data structure to a byte stream as an …\nSerializes a <code>Value</code> to a byte stream as an XML encoded …\nSerializes to a byte stream as an XML encoded plist, using …\nSerializes a <code>Value</code> to a stream, using custom …\nConverts the <code>Data</code> to an XML plist (Base-64) string.\nConverts the <code>Date</code> to an XML plist date string.\nRepresents a plist dictionary type.\nAn owning iterator over a plist::Dictionary’s entries.\nAn iterator over a plist::Dictionary’s entries.\nA mutable iterator over a plist::Dictionary’s entries.\nAn iterator over a plist::Dictionary’s keys.\nAn iterator over a plist::Dictionary’s values.\nA mutable iterator over a plist::Dictionary’s values.\nClears the dictionary, removing all values.\nReturns true if the dictionary contains a value for the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns a reference to the value corresponding to the key.\nReturns a mutable reference to the value corresponding to …\nInserts a key-value pair into the dictionary.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true if the dictionary contains no elements.\nGets an iterator over the entries of the dictionary.\nGets a mutable iterator over the entries of the dictionary.\nGets an iterator over the keys of the dictionary.\nReturns the number of elements in the dictionary.\nMakes a new empty <code>Dictionary</code>.\nRemoves a key from the dictionary, returning the value at …\nScan through each key-value pair in the map and keep those …\nSort the dictionary keys.\nGets an iterator over the values of the dictionary.\nGets an iterator over mutable values of the dictionary.")