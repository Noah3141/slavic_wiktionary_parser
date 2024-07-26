use super::*;

#[test]
fn select_from_with_to() {
    let text = String::from("This is a test string with {{macro}} and some more text.");
    let result = select_from(&text, "{{macro", "}}");
    assert_eq!(result, Some(""));
}

#[test]
fn select_from_without_to() {
    let text = String::from("This is a test string with {{macro}} and some more text.");
    let result = select_from(&text, "{{macro", "missing");
    assert_eq!(result, Some("}} and some more text."));
}

#[test]
fn select_from_no_match() {
    let text = String::from("This is a test string with no macros.");
    let result = select_from(&text, "{{macro", "}}");
    assert_eq!(result, None);
}

#[test]
fn select_from_to_at_end() {
    let text = String::from("This is a test string with {{macro}}");
    let result = select_from(&text, "{{macro", "missing");
    assert_eq!(result, Some("}}"));
}

#[test]
fn select_from_from_not_found() {
    let text = String::from("This is a test string with no macros.");
    let result = select_from(&text, "{{missing", "}}");
    assert_eq!(result, None);
}

#[test]
fn select_from_with_nested_macros() {
    let text = String::from("This has {{nested {{macro}} inside}}.");
    let result = select_from(&text, "{{nested", "}}");
    assert_eq!(result, Some(" {{macro"));
}

#[test]
fn extracts_page_title() {
    let text = String::from("kfnkaosdfoi <page>Фу</page>asdas");
    let result = select_from(&text, "<page>", "</page>");
    assert_eq!(result, Some("Фу"));
}



#[test]
fn select_language_section() {
    let text = String::from(r#"==Belarusian==
{{wp|lang=be}}

===Alternative forms===
* {{alt|be|koszka|koška||Łacinka}}

===Etymology===
{{inh+|be|zle-ort|ко́шка}}, from {{inh|be|orv|ко́шька}}, from unattested {{m|orv|*ко́чька}}, from {{inh|be|sla-pro|*kòťьka}}, from {{m|sla-pro|*kòťь}}, from {{m|sla-pro|*kòtъ}}. Cognate with {{cog|ru|ко́шка}} and {{cog|uk|кі́шка}}.

===Pronunciation===
* {{be-IPA|ко́шка}}
* {{rhymes|be|oʂka|s=2}}
* {{audio|be|LL-Q9091_(bel)-Ssvb-кошка.wav}}

===Noun===
{{be-noun|ко́шка&lt;*.anml&gt;}}

# female [[cat]] {{gl|domesticated species}}
#: {{syn|be|ко́тка}}
# [[cat]] {{gl|member of the cat family {{taxfmt|Felidae|family}}}}

====Declension====
{{be-ndecl|ко́шка&lt;*.anml&gt;}}

===See also===
* {{l|be|кот}}

===References===
* {{R:be:slounik.org}}
* {{R:be:Skarnik}}

{{topics|be|Cats}}

==Macedonian==

===Pronunciation===
* {{mk-IPA}}

===Verb===
{{mk-verb|impf}}

# {{lb|mk|transitive}} to [[kick]]
# {{lb|mk|transitive|figurative}} to [[belittle]], [[disparage]]

====Conjugation====
{{mk-conj-а|impf|кошк}}"#);
    let result = select_from(&text, "==Belarusian==", "==Macedonian==");
    assert_eq!(result, Some(r#"
{{wp|lang=be}}

===Alternative forms===
* {{alt|be|koszka|koška||Łacinka}}

===Etymology===
{{inh+|be|zle-ort|ко́шка}}, from {{inh|be|orv|ко́шька}}, from unattested {{m|orv|*ко́чька}}, from {{inh|be|sla-pro|*kòťьka}}, from {{m|sla-pro|*kòťь}}, from {{m|sla-pro|*kòtъ}}. Cognate with {{cog|ru|ко́шка}} and {{cog|uk|кі́шка}}.

===Pronunciation===
* {{be-IPA|ко́шка}}
* {{rhymes|be|oʂka|s=2}}
* {{audio|be|LL-Q9091_(bel)-Ssvb-кошка.wav}}

===Noun===
{{be-noun|ко́шка&lt;*.anml&gt;}}

# female [[cat]] {{gl|domesticated species}}
#: {{syn|be|ко́тка}}
# [[cat]] {{gl|member of the cat family {{taxfmt|Felidae|family}}}}

====Declension====
{{be-ndecl|ко́шка&lt;*.anml&gt;}}

===See also===
* {{l|be|кот}}

===References===
* {{R:be:slounik.org}}
* {{R:be:Skarnik}}

{{topics|be|Cats}}

"#));

}