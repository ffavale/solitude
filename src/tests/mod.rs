// This file is part of Solitude.
//     Solitude is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//     Solitude is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//     You should have received a copy of the GNU General Public License along with Solitude. If not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
use crate::*;

#[test]
fn deck_shuffle() {
    assert_ne!(new_standard_deck(), new_standard_deck().shuffle());
    assert_ne!(new_italian_deck(), new_italian_deck().shuffle());
}

#[test]
fn shuffle_hundred() {
    for _ in 0..100 {
        assert_ne!(new_standard_deck(), new_standard_deck().shuffle());
        assert_ne!(new_italian_deck(), new_italian_deck().shuffle());
    }
}

#[test]
fn shuffle_e3() {
    for _ in 0..1_000 {
        assert_ne!(new_standard_deck(), new_standard_deck().shuffle());
        assert_ne!(new_italian_deck(), new_italian_deck().shuffle());
    }
}
