use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;

pub mod option_i64 {
    use super::*;
    use serde_with::DisplayFromStr;

    pub fn serialize<S>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Helper<'a>(#[serde_as(as = "DisplayFromStr")] &'a i64);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[serde_as]
        #[derive(Deserialize)]
        struct Helper(#[serde_as(as = "DisplayFromStr")] i64);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

pub mod option_u64 {
    use super::*;
    use serde_with::DisplayFromStr;

    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Helper<'a>(#[serde_as(as = "DisplayFromStr")] &'a u64);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[serde_as]
        #[derive(Deserialize)]
        struct Helper(#[serde_as(as = "DisplayFromStr")] u64);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

pub mod option_bytes {
    use super::*;
    use prost::bytes::Bytes;
    use serde_with::base64::Base64;

    pub fn serialize<S>(value: &Option<Bytes>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Helper<'a>(#[serde_as(as = "Base64")] &'a Bytes);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Bytes>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[serde_as]
        #[derive(Deserialize)]
        struct Helper(#[serde_as(as = "Base64")] Bytes);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

pub mod option_enum {
    use super::*;
    use serde::de::DeserializeOwned;
    use serde_with::TryFromInto;

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct Helper<T: Serialize + DeserializeOwned + TryFrom<i32, Error = String> + Into<i32>>(
        #[serde_as(as = "TryFromInto<T>")] i32,
        std::marker::PhantomData<T>,
    );

    pub fn serialize<
        'a,
        T: Serialize + DeserializeOwned + TryFrom<i32, Error = String> + Into<i32> + Copy,
        S,
    >(
        value: &'a Option<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value
            .as_ref()
            .map(|x| Helper((*x).into(), std::marker::PhantomData::<T>))
            .serialize(serializer)
    }

    pub fn deserialize<
        'de,
        T: Serialize + DeserializeOwned + TryFrom<i32, Error = String> + Into<i32> + Copy,
        D,
    >(
        deserializer: D,
    ) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper<T>(T);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}
