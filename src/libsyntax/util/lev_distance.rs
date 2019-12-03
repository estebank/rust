use std::cmp;
use crate::symbol::Symbol;
use unicode_skeleton::{confusable, UnicodeSkeleton};

#[cfg(test)]
mod tests;

/// Finds the Levenshtein distance between two strings
pub fn lev_distance(a: &str, b: &str) -> usize {
    // cases which don't require further computation
    if a.is_empty() {
        return b.chars().count();
    } else if b.is_empty() {
        return a.chars().count();
    }

    let mut dcol: Vec<_> = (0..=b.len()).collect();
    let mut t_last = 0;

    for (i, sc) in a.chars().enumerate() {
        let mut current = i;
        dcol[0] = current + 1;

        for (j, tc) in b.chars().enumerate() {
            let next = dcol[j + 1];
            if sc == tc {
                dcol[j + 1] = current;
            } else {
                dcol[j + 1] = cmp::min(current, next);
                dcol[j + 1] = cmp::min(dcol[j + 1], dcol[j]) + 1;
            }
            current = next;
            t_last = j;
        }
    }
    dcol[t_last + 1]
}

/// Finds the Levenshtein distance between two strings. If the two strings contain characters that
/// could be confused in an unicode context, we return the Levenshtein distance of the `skeleton`s
/// of both strings instead.
pub fn skeleton_aware_lev_distance(a: &str, b: &str) -> usize {
    if a != b && confusable(a, b) { // skeleton(a) == skeleton(b)
        // `lev_distance(skeleton(a), skeleton(b))` would always be 0, this is an attempt at giving
        // a more expected result.
        cmp::min(
            cmp::min(
                lev_distance(a, &b.skeleton_chars().collect::<String>()),
                lev_distance(&a.skeleton_chars().collect::<String>(), b),
            ),
            lev_distance(a, b),
        )
    } else {
        lev_distance(a, b)
    }
}

/// Finds the best match for a given word in the given iterator
///
/// As a loose rule to avoid the obviously incorrect suggestions, it takes
/// an optional limit for the maximum allowable edit distance, which defaults
/// to one-third of the given word.
///
/// Besides Levenshtein, we use case insensitive comparison to improve accuracy on an edge case with
/// a lower(upper)case letters mismatch.
pub fn find_best_match_for_name<'a, T>(
    iter_names: T,
    lookup: &str,
    dist: Option<usize>,
) -> Option<Symbol>
    where T: Iterator<Item = &'a Symbol>
{
    let max_dist = dist.map_or_else(|| cmp::max(lookup.len(), 3) / 3, |d| d);

    let (case_insensitive_match, levenstein_match) = iter_names
        .filter_map(|&name| {
            let dist = skeleton_aware_lev_distance(lookup, &name.as_str());
            if dist <= max_dist {
                Some((name, dist))
            } else {
                None
            }
        })
        // Here we are collecting the next structure:
        // (case_insensitive_match, (levenstein_match, levenstein_distance))
        .fold((None, None), |result, (candidate, dist)| (
            if candidate.as_str().to_uppercase() == lookup.to_uppercase() {
                Some(candidate)
            } else {
                result.0
            },
            match result.1 {
                None => Some((candidate, dist)),
                Some((c, d)) => Some(if dist < d { (candidate, dist) } else { (c, d) }),
            },
        ));

    match (case_insensitive_match, levenstein_match) {
        // Exact case insensitive match has a higher priority.
        (Some(candidate), _) => Some(candidate),
        (None, Some((candidate, _))) => Some(candidate),
        _ => None,
    }
}
