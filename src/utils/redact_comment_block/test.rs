use super::*;

#[test]
fn comment_block_redation() {
    let mut page: Box<[u8]> = Box::new(*br#"<page>
    <title>User:Tobin Richard~enwiktionary</title>
    <ns>2</ns>
    <id>26</id>
    <revision>
      <id>32613994</id>
      <parentid>241848</parentid>
      <timestamp>2015-04-19T13:18:17Z</timestamp>
      <contributor>
        <username>Maintenance script</username>
        <id>2431690</id>
      </contributor>
      <minor />
      <comment>Maintenance script moved page [[User:Tobin Richard]] to [[User:Tobin Richard~enwiktionary]] without leaving a redirect: Automatically moved page while renaming the user &quot;[[Special:CentralAuth/Tobin Richard|Tobin Richard]]&quot; to &quot;[[Special:CentralAuth/Tob...</comment>
      <origin>32613994</origin>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes="150" sha1="3gmacjump5fspu9bfq5slkko2w0dp2y" xml:space="preserve">Squat australian with poor spelling, bad [[grammar]] and limited [[knowledge]].... But a disire to see the [[Wikipedia]] and [[Wiktionary]] advance :)</text>
      <sha1>3gmacjump5fspu9bfq5slkko2w0dp2y</sha1>
    </revision>
  </page>"#);
    
    redact_comment_block(&mut page);

    assert!(!String::from_utf8(page.to_vec()).expect("utf-8").contains("<comment>"))
}
