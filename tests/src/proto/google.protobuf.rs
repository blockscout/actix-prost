/// A Timestamp represents a point in time independent of any time zone or local
/// calendar, encoded as a count of seconds and fractions of seconds at
/// nanosecond resolution. The count is relative to an epoch at UTC midnight on
/// January 1, 1970, in the proleptic Gregorian calendar which extends the
/// Gregorian calendar backwards to year one.
///
/// All minutes are 60 seconds long. Leap seconds are "smeared" so that no leap
/// second table is needed for interpretation, using a [24-hour linear
/// smear](<https://developers.google.com/time/smear>).
///
/// The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By
/// restricting to that range, we ensure that we can convert to and from [RFC
/// 3339](<https://www.ietf.org/rfc/rfc3339.txt>) date strings.
///
/// # Examples
///
/// Example 1: Compute Timestamp from POSIX `time()`.
///
///      Timestamp timestamp;
///      timestamp.set_seconds(time(NULL));
///      timestamp.set_nanos(0);
///
/// Example 2: Compute Timestamp from POSIX `gettimeofday()`.
///
///      struct timeval tv;
///      gettimeofday(&tv, NULL);
///
///      Timestamp timestamp;
///      timestamp.set_seconds(tv.tv_sec);
///      timestamp.set_nanos(tv.tv_usec * 1000);
///
/// Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.
///
///      FILETIME ft;
///      GetSystemTimeAsFileTime(&ft);
///      UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;
///
///      // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
///      // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
///      Timestamp timestamp;
///      timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
///      timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
///
/// Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.
///
///      long millis = System.currentTimeMillis();
///
///      Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
///          .setNanos((int) ((millis % 1000) * 1000000)).build();
///
///
/// Example 5: Compute Timestamp from Java `Instant.now()`.
///
///      Instant now = Instant.now();
///
///      Timestamp timestamp =
///          Timestamp.newBuilder().setSeconds(now.getEpochSecond())
///              .setNanos(now.getNano()).build();
///
///
/// Example 6: Compute Timestamp from current time in Python.
///
///      timestamp = Timestamp()
///      timestamp.GetCurrentTime()
///
/// # JSON Mapping
///
/// In JSON format, the Timestamp type is encoded as a string in the
/// [RFC 3339](<https://www.ietf.org/rfc/rfc3339.txt>) format. That is, the
/// format is "{year}-{month}-{day}T{hour}:{min}:{sec}\[.{frac_sec}\]Z"
/// where {year} is always expressed using four digits while {month}, {day},
/// {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
/// seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
/// are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
/// is required. A proto3 JSON serializer should always use UTC (as indicated by
/// "Z") when printing the Timestamp type and a proto3 JSON parser should be
/// able to accept both UTC and other timezones (as indicated by an offset).
///
/// For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
/// 01:30 UTC on January 15, 2017.
///
/// In JavaScript, one can convert a Date object to this format using the
/// standard
/// \[toISOString()\](<https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString>)
/// method. In Python, a standard `datetime.datetime` object can be converted
/// to this format using
/// \[`strftime`\](<https://docs.python.org/2/library/time.html#time.strftime>) with
/// the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one can use
/// the Joda Time's \[`ISODateTimeFormat.dateTime()`\](
/// <http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime%2D%2D>
/// ) to obtain a formatter capable of generating timestamps in this format.
///
///
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
/// `Any` contains an arbitrary serialized protocol buffer message along with a
/// URL that describes the type of the serialized message.
///
/// Protobuf library provides support to pack/unpack Any values in the form
/// of utility functions or additional generated methods of the Any type.
///
/// Example 1: Pack and unpack a message in C++.
///
///      Foo foo = ...;
///      Any any;
///      any.PackFrom(foo);
///      ...
///      if (any.UnpackTo(&foo)) {
///        ...
///      }
///
/// Example 2: Pack and unpack a message in Java.
///
///      Foo foo = ...;
///      Any any = Any.pack(foo);
///      ...
///      if (any.is(Foo.class)) {
///        foo = any.unpack(Foo.class);
///      }
///
/// Example 3: Pack and unpack a message in Python.
///
///      foo = Foo(...)
///      any = Any()
///      any.Pack(foo)
///      ...
///      if any.Is(Foo.DESCRIPTOR):
///        any.Unpack(foo)
///        ...
///
/// Example 4: Pack and unpack a message in Go
///
///       foo := &pb.Foo{...}
///       any, err := anypb.New(foo)
///       if err != nil {
///         ...
///       }
///       ...
///       foo := &pb.Foo{}
///       if err := any.UnmarshalTo(foo); err != nil {
///         ...
///       }
///
/// The pack methods provided by protobuf library will by default use
/// 'type.googleapis.com/full.type.name' as the type URL and the unpack
/// methods only use the fully qualified type name after the last '/'
/// in the type URL, for example "foo.bar.com/x/y.z" will yield type
/// name "y.z".
///
///
/// JSON
///
/// The JSON representation of an `Any` value uses the regular
/// representation of the deserialized, embedded message, with an
/// additional field `@type` which contains the type URL. Example:
///
///      package google.profile;
///      message Person {
///        string first_name = 1;
///        string last_name = 2;
///      }
///
///      {
///        "@type": "type.googleapis.com/google.profile.Person",
///        "firstName": <string>,
///        "lastName": <string>
///      }
///
/// If the embedded message type is well-known and has a custom JSON
/// representation, that representation will be embedded adding a field
/// `value` which holds the custom JSON in addition to the `@type`
/// field. Example (for message \[google.protobuf.Duration][\]):
///
///      {
///        "@type": "type.googleapis.com/google.protobuf.Duration",
///        "value": "1.212s"
///      }
///
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Any {
    /// A URL/resource name that uniquely identifies the type of the serialized
    /// protocol buffer message. This string must contain at least
    /// one "/" character. The last segment of the URL's path must represent
    /// the fully qualified name of the type (as in
    /// `path/google.protobuf.Duration`). The name should be in a canonical form
    /// (e.g., leading "." is not accepted).
    ///
    /// In practice, teams usually precompile into the binary all types that they
    /// expect it to use in the context of Any. However, for URLs which use the
    /// scheme `http`, `https`, or no scheme, one can optionally set up a type
    /// server that maps type URLs to message definitions as follows:
    ///
    /// * If no scheme is provided, `https` is assumed.
    /// * An HTTP GET on the URL must yield a \[google.protobuf.Type][\]
    ///    value in binary format, or produce an error.
    /// * Applications are allowed to cache lookup results based on the
    ///    URL, or have them precompiled into a binary to avoid any
    ///    lookup. Therefore, binary compatibility needs to be preserved
    ///    on changes to types. (Use versioned type names to manage
    ///    breaking changes.)
    ///
    /// Note: this functionality is not currently available in the official
    /// protobuf release, and it is not used for type URLs beginning with
    /// type.googleapis.com.
    ///
    /// Schemes other than `http`, `https` (or the empty scheme) might be
    /// used with implementation specific semantics.
    ///
    #[prost(string, tag = "1")]
    pub type_url: ::prost::alloc::string::String,
    /// Must be a valid serialized protocol buffer of the above specified type.
    #[prost(bytes = "bytes", tag = "2")]
    pub value: ::prost::bytes::Bytes,
}
