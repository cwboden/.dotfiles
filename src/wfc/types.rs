use enum_iterator::IntoEnumIterator;

#[derive(Clone, Copy, Debug, Eq, Hash, IntoEnumIterator, PartialEq)]
enum Rotation {
    Unchanged,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

#[derive(Clone, Copy, Debug, Eq, Hash, IntoEnumIterator, PartialEq)]
enum Mirrored {
    Unchanged,
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Transform {
    rotation: Rotation,
    mirrored: Mirrored,
}

impl Transform {
    pub fn iter() -> impl Iterator<Item = Transform> {
        [
            Transform {
                rotation: Rotation::Unchanged,
                mirrored: Mirrored::Unchanged,
            },
            Transform {
                rotation: Rotation::Clockwise90,
                mirrored: Mirrored::Unchanged,
            },
            Transform {
                rotation: Rotation::Clockwise180,
                mirrored: Mirrored::Unchanged,
            },
            Transform {
                rotation: Rotation::Clockwise270,
                mirrored: Mirrored::Unchanged,
            },
            Transform {
                rotation: Rotation::Unchanged,
                mirrored: Mirrored::Vertical,
            },
            Transform {
                rotation: Rotation::Clockwise90,
                mirrored: Mirrored::Vertical,
            },
            Transform {
                rotation: Rotation::Clockwise180,
                mirrored: Mirrored::Vertical,
            },
            Transform {
                rotation: Rotation::Clockwise270,
                mirrored: Mirrored::Vertical,
            },
            Transform {
                rotation: Rotation::Unchanged,
                mirrored: Mirrored::Horizontal,
            },
            Transform {
                rotation: Rotation::Clockwise90,
                mirrored: Mirrored::Horizontal,
            },
            Transform {
                rotation: Rotation::Clockwise180,
                mirrored: Mirrored::Horizontal,
            },
            Transform {
                rotation: Rotation::Clockwise270,
                mirrored: Mirrored::Horizontal,
            },
        ]
        .iter()
        .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_all_transforms_exist() {
        let mut expected_transforms = HashSet::new();
        for rotation in Rotation::into_enum_iter() {
            for mirrored in Mirrored::into_enum_iter() {
                expected_transforms.insert(Transform { rotation, mirrored });
            }
        }

        let actual_transforms: HashSet<Transform> = Transform::iter().collect();
        assert_eq!(actual_transforms, expected_transforms);
    }
}
