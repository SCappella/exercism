use unicode_reverse::reverse_grapheme_clusters_in_place;

#[inline]
pub fn reverse(input: &str) -> String {
    let mut output = String::from(input);
    reverse_grapheme_clusters_in_place(&mut output);
    output
}
