foo.trimLeft();
foo.trimRight();
trimLeft.trimRight();
foo.trimLeft.trimRight();
"foo".trimLeft();
foo
	// comment
	.trimRight /* comment */
	/* comment */
	();
foo?.trimLeft();

// Substring
foo.substr()
foo?.substr()
foo.bar?.substring()
foo?.[0]?.substring()
foo.bar.substr?.()
foo.bar?.substring?.()
foo.bar?.baz?.substr()
foo.bar?.baz.substring()
foo.bar.baz?.substr()
"foo".substr()
"foo".substr(bar.length, Math.min(baz, 100)) // "foo".slice(bar.length, bar.length + Math.min(baz, 100))
"foo".substr(1, "abc".length) // "foo".slice(1, 1 + "abc".length)
"foo".substr("1", 2)
"foo".substring(length, 0) // "foo".slice(0, Math.max(0, length))
foo.substring(start) // foo.slice(Math.max(0, start))
foo.substring(start, end)
"foo".substring(1, 3)
// Extra arguments
foo.substring(1, 2, 3)
