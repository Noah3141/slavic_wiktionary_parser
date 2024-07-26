use super::*;

#[test]
fn selected_contains_from_in_return() {
    let text = String::from("<page>\n<title>кошка</title>\n<ns>0</ns>\n<id>7520</id>\n<revision>\n<id>80457929</id>\n<parentid>79591248</parentid>\n<timestamp>2024-06-19T06:36:35Z</timestamp>\n<contributor>\n<username>Merrahtar</username>\n<id>3477317</id>\n</contributor>\n\n");
    let from = "кошка";
    let result = select_unto_language_header(&text, from).unwrap();
    assert!(result.contains("кошка"));
}

#[test]
fn select_unto_from_not_found_should_err() {
    let text = String::from("<page>\n<title>кошка</title>\n<ns>0</ns>\n<id>7520</id>\n<revision>\n<id>80457929</id>\n<parentid>79591248</parentid>\n<timestamp>2024-06-19T06:36:35Z</timestamp>\n<contributor>\n<username>Merrahtar</username>\n<id>3477317</id>\n</contributor>\n\n");
    let from = "dog";
    let result = select_unto_language_header(&text, from);
    assert_eq!(result, Err(String::from("Starting text was not found")));
}

#[test]
fn select_unto_language_header_end_of_text() {
    let text = String::from("<page>\n<title>кошка</title>\n<ns>0</ns>\n<id>7520</id>\n<revision>\n<id>80457929</id>\n<parentid>79591248</parentid>\n<timestamp>2024-06-19T06:36:35Z</timestamp>\n<contributor>\n<username>Merrahtar</username>\n<id>3477317</id>\n</contributor>\n==Section1==\nContent of section 1\n==Section2==\nContent of section 2\n===Subsection1===\nContent of subsection 1\n==Section3==\nContent of section 3</page>");
    let from = "==Section2==";
    let result = select_unto_language_header(&text, from).unwrap();
    assert!(result.contains("Section2"));
    assert!(!result.contains("==Section3=="));
}

#[test]
fn real_example() {
    let text = String::from("===================================================
<page>
    <title>газе</title>
    <ns>0</ns>
    <id>5171528</id>
    <revision>
      <id>80322024</id>
      <parentid>71908908</parentid>
      <timestamp>2024-06-18T05:31:45Z</timestamp>
      <contributor>
        <username>WingerBot</username>
        <id>2024159</id>
      </contributor>
      <minor />
      \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0
      <origin>80322024</origin>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes=\"372\" sha1=\"adyq8rtw0ihpjcwdrf6ptminxn7os4v\" xml:space=\"preserve\">{{also|ГАЗе}}
==Macedonian==

===Pronunciation===
* {{mk-IPA}}

===Noun===
{{mk-noun|n|газиња|dim=газенце}}

# {{diminutive of|mk|газ}}

====Declension====
{{mk-decl-noun-n-е-иња|газ}}

==Russian==

===Pronunciation===
* {{ru-IPA|га́зе|pos=pre}}

===Noun===
{{head|ru|noun form|head=га́зе|g=m-in}}

# {{inflection of|ru|газ||pre|s}}</text>
      <sha1>adyq8rtw0ihpjcwdrf6ptminxn7os4v</sha1>
    </revision>
  </page>
===================================================
      ");
    let result = select_unto_language_header(&text, "==Russian==").unwrap();
    assert!(result.contains("* {{ru-IPA|га́зе|pos=pre}}"));
    assert!(!result.contains("* {{mk-IPA}}"));
}


#[test]
fn real_example_2() {
    let text = String::from("{{also|зѫба}}
==Belarusian==

===Pronunciation===
* {{be-IPA|зу́ба}}

===Noun===
{{head|be|noun form|head=зу́ба|g=m-in}}

# {{inflection of|be|зуб||gen|s}}

==Russian==

===Pronunciation===
* {{ru-IPA|зу́ба}}

===Noun===
{{head|ru|noun form|head=зу́ба|g=m-in}}

# {{inflection of|ru|зу́б||gen|s}}

==Serbo-Croatian==

===Pronunciation 1===
{{rfc-pron-n|sh|Pronunciation 1}}
{{rfp|sh}}

====Noun====
{{sh-noun-form}}

# {{inflection of|sh|зу̑б||gen|s}}

===Pronunciation 2===
* {{IPA|sh|/zǔːbaː/}}

====Noun====
{{sh-noun-form|head=зу́ба̄}}

# {{inflection of|sh|зу̑б||gen|p}}

==Ukrainian==

===Pronunciation===
* {{uk-IPA|зу́ба}}

===Noun===
{{head|uk|noun form|head=зу́ба|g=m-in}}

# {{inflection of|uk|зуб||gen|s}}</text>
      <sha1>svsgpjh2rect1yyvy5p0nb3e59cduj5</sha1>
    </revision>
  </page>");
    let russian = select_unto_language_header(&text, "==Russian==").unwrap();
    
    assert!(russian.contains("* {{ru-IPA|зу́ба}}"));
    assert!(russian.contains("# {{inflection of|ru|зу́б||gen|s}}"));
    assert!(!russian.contains("{{rfc-pron-n|sh|Pronunciation 1}}"));
    
    let ukrainian = select_unto_language_header(&text, "==Ukrainian==").unwrap();
    assert!(ukrainian.contains("* {{uk-IPA|зу́ба}}"));
    assert!(!ukrainian.contains("* {{ru-IPA|зу́ба}}"));
    assert!(!ukrainian.contains("* {{be-IPA|зу́ба}}"));
    
    
    let belarusian = select_unto_language_header(&text, "==Belarusian==").unwrap();
    assert!(belarusian.contains("* {{be-IPA|зу́ба}}"));
    assert!(!belarusian.contains("* {{ru-IPA|зу́ба}}"));

}