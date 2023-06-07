use std::collections::BTreeMap;

use crate::extension::Extensions;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// # OpenApi
/// This is the root document object of the OpenAPI document.
/// see https://swagger.io/specification/v3/
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenApi {
    /// This string MUST be the semantic version number of the OpenAPI Specification version that the OpenAPI document uses.
    pub openapi: String,
    /// Provides metadata about the API.
    pub info: Info,
    /// An array of Server Objects, which provide connectivity information to a target server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,

    pub paths: Paths,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityRequirement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<ExternalDoc>,
}
/// ## Info
/// The object provides metadata about the API.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Info {
    ///  The title of the application.
    pub title: String,
    /// A short description of the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The Terms of Service for the API.
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    /// Provides the version of the application API (not to be confused with the specification version).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### Contact
/// Contact information for the exposed API.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information. MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The email address of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### License
/// License information for the exposed API.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct License {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API. MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ## Server
/// An object representing a Server.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Server {
    ///  A URL to the target host.
    pub url: String,
    /// An optional string describing the host designated by the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A map between a variable name and its value. The value is used for substitution in the server's URL template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, ServerVariable>>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### ServerVariable
/// An object representing a Server Variable for server URL template substitution.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerVariable {
    /// An enumeration of string values to be used if the substitution options are from a limited set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    /// The default value to use for substitution, which SHALL be sent if an alternate value is *not* supplied.
    pub default: String,
    /// An optional description for the server variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(flatten)]
    pub extensions: Extensions,
}
/// # Components
/// Holds a set of reusable objects for different aspects of the OAS. All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Components {
    /// An object to hold reusable Schema Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<BTreeMap<String, RefOrObject<Schema>>>,
    /// An object to hold reusable Response Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Responses>,
    /// An object to hold reusable Parameter Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, RefOrObject<Parameter>>>,
    /// An object to hold reusable Example Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<BTreeMap<String, RefOrObject<Example>>>,
    /// An object to hold reusable Request Body Objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestBodies")]
    pub request_bodies: Option<BTreeMap<String, RefOrObject<RequestBody>>>,
    /// An object to hold reusable Header Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, RefOrObject<Header>>>,
    /// An object to hold reusable Security Scheme Objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securitySchemes")]
    pub security_schemes: Option<BTreeMap<String, RefOrObject<SecurityScheme>>>,
    /// An object to hold reusable Link Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<BTreeMap<String, RefOrObject<Link>>>,
    /// An object to hold reusable Callback Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<BTreeMap<String, RefOrObject<Callback>>>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### Paths
/// Holds the relative paths to the individual endpoints and their operations.
///
/// The path is appended to the URL from the Server Object in order to construct the full URL.
///
/// The Paths MAY be empty, due to [ACL constraints](https://swagger.io/specification/v3/#security-filtering).
pub type Paths = BTreeMap<String, PathItem>;

/// ### Path Item
/// Describes the operations available on a single path.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PathItem {
    /// Allows for an external definition of this path item. The referenced structure MUST be in the format of a Path Item Object.
    /// If there are conflicts between the referenced definition and this Path Item's definition, the behavior is undefined.
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// An optional, string summary, intended to apply to all operations in this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// An optional, string description, intended to apply to all operations in this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    /// A definition of a TRACE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,
    /// An alternative server array to service all operations in this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,

    /// A list of parameters that are applicable for all the operations described under this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<RefOrObject<Parameter>>>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### Operation
/// Describes a single API operation on a path.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Operation {
    /// A list of tags for API documentation control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<ExternalDoc>,
    /// Unique string used to identify the operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "operationId")]
    pub operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<RefOrObject<Parameter>>>,
    /// The request body applicable for this operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestBody")]
    pub request_body: Option<RefOrObject<RequestBody>>,
    /// The list of possible responses as they are returned from executing this operation.
    pub responses: Responses,
    /// A map of possible out-of band callbacks related to the parent operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<BTreeMap<String, RefOrObject<Callback>>>,
    /// Declares this operation to be deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// A declaration of which security mechanisms can be used for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
    /// An alternative server array to service this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### External Documentation
/// Allows referencing an external resource for extended documentation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalDoc {
    /// A short description of the target documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// he URL for the target documentation.
    pub url: String,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ## Parameter
///
/// Describes a single operation parameter.
/// A unique parameter is defined by a combination of a name and location.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// The location of the parameter. Possible values are "query", "header", "path" or "cookie".
    pub r#in: String,
    /// A brief description of the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// Sets the ability to pass empty-valued parameters.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>,

    ///Describes how the parameter value will be serialized depending on the type of the parameter value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// When this is true, parameter values of type array or object generate separate parameters for each value of the array or key-value pair of the map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,

    /// Determines whether the parameter value SHOULD allow reserved characters
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowReserved")]
    pub allow_reserved: Option<bool>,

    /// The schema defining the type used for the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOrObject<Schema>>,
    /// Example of the parameter's potential value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    /// Examples of the parameter's potential value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<BTreeMap<String, RefOrObject<Example>>>,
    /// A map containing the representations for the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<BTreeMap<String, Media>>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### Request Body
/// Describes a single request body.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RequestBody {
    /// A brief description of the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The content of the request body.
    /// The key is a media type or media type range and the value describes it.
    pub content: BTreeMap<String, Media>,
    /// Determines if the request body is required in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ## Responses
/// A container for the expected responses of an operation. The container maps a HTTP response code to the expected response.
pub type Responses = BTreeMap<String, RefOrObject<Response>>;

/// ### Reponse
/// Describes a single response from an API Operation, including design-time, static links to operations based on the response.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Response {
    /// A short description of the response.
    pub description: String,
    /// Maps a header name to its definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, RefOrObject<Header>>>,
    /// A map containing descriptions of potential response payloads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<BTreeMap<String, Media>>,
    /// A map of operations links that can be followed from the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<BTreeMap<String, RefOrObject<Link>>>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### Media
/// Each Media Type Object provides schema and examples for the media type identified by its key.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Media {
    /// The schema defining the content of the request, response, or parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOrObject<Schema>>,
    /// Example of the media type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    /// Examples of the media type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<BTreeMap<String, RefOrObject<Example>>>,
    /// A map between a property name and its encoding information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<BTreeMap<String, Encoding>>,
}

/// ### Encoding
/// A single encoding definition applied to a single schema property.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Encoding {
    /// The Content-Type for encoding a specific property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// A map allowing additional information to be provided as headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, RefOrObject<Header>>>,
    /// Describes how a specific property value will be serialized depending on its type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// When this is true, property values of type array or object generate separate parameters for each value of the array, or key-value-pair of the map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,
    /// Determines whether the parameter value SHOULD allow reserved characters, as defined by RFC3986 :/?#[]@!$&'()*+,;= to be included without percent-encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
}

/// ### Callback
/// A map of possible out-of band callbacks related to the parent operation.
pub type Callback = BTreeMap<String, PathItem>;

/// ### Example
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Example {
    /// Short description for the example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Long description for the example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Embedded literal example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// A URL that points to the literal example.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalValue")]
    pub external_value: Option<String>,
}

/// ### Link
/// The Link object represents a possible design-time link for a response.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Link {
    /// A relative or absolute URI reference to an OAS operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "operationRef")]
    pub operation_ref: Option<String>,
    /// The name of an existing, resolvable OAS operation, as defined with a unique operationId. This field is mutually exclusive of the operationRef field.
    #[serde(skip_serializing_if = "Option::is_none", rename = "operationId")]
    pub operation_id: Option<String>,
    /// A map representing parameters to pass to an operation as specified with operationId or identified via operationRef.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, Value>>,
    /// A literal value or {expression} to use as a request body when calling the target operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestBody")]
    pub request_body: Option<Value>,
    /// A description of the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A server object to be used by the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}
/// ### Header
/// The Header Object follows the structure of the Parameter Object with the following changes
/// see https://swagger.io/specification/v3/#header-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Header {
    /// A brief description of the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// Sets the ability to pass empty-valued parameters.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>,

    ///Describes how the parameter value will be serialized depending on the type of the parameter value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// When this is true, parameter values of type array or object generate separate parameters for each value of the array or key-value pair of the map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,

    /// Determines whether the parameter value SHOULD allow reserved characters
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowReserved")]
    pub allow_reserved: Option<bool>,

    /// The schema defining the type used for the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOrObject<Schema>>,
    /// Example of the parameter's potential value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    /// Examples of the parameter's potential value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<BTreeMap<String, RefOrObject<Example>>>,
    /// A map containing the representations for the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<BTreeMap<String, Media>>,

    #[serde(flatten)]
    pub extensions: Extensions,
}
/// ## Tag
/// Adds metadata to a single tag that is used by the Operation Object.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// A short description for the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<ExternalDoc>,

    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### Reference
/// A simple object to allow referencing other components in the specification, internally and externally.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Reference {
    /// The reference string.
    #[serde(rename = "$ref")]
    pub reference: String,
}

/// ### Schema
/// The Schema Object allows the definition of input and output data types.
/// ser https://swagger.io/specification/v3/#schema-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub r#enum: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multipleOf")]
    pub multiple_of: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxLength")]
    pub max_length: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "minLength")]
    pub min_length: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxItems")]
    pub max_items: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "minItems")]
    pub min_items: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<RefOrObject<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, RefOrObject<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxProperties")]
    pub max_properties: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "minProperties")]
    pub min_properties: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "allOf")]
    pub all_of: Option<Vec<Box<RefOrObject<Schema>>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "oneOf")]
    pub one_of: Option<Vec<Box<RefOrObject<Schema>>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "anyOf")]
    pub any_of: Option<Vec<Box<RefOrObject<Schema>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<RefOrObject<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "additionalProperties"
    )]
    pub additional_properties: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    /// Adds support for polymorphism.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<Discriminator>,
    /// Relevant only for Schema "properties" definitions.
    #[serde(skip_serializing_if = "Option::is_none", rename = "readOnly")]
    pub read_only: Option<bool>,
    /// Relevant only for Schema "properties" definitions.
    #[serde(skip_serializing_if = "Option::is_none", rename = "writeOnly")]
    pub write_only: Option<bool>,
    /// This MAY be used only on properties schemas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml: Option<XML>,
    /// Additional external documentation for this schema.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<ExternalDoc>,
    /// A free-form property to include an example of an instance for this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    /// Specifies that a schema is deprecated and SHOULD be transitioned out of usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

/// ### Discriminator
/// When request bodies or response payloads may be one of a number of different schemas, a discriminator object can be used to aid in serialization, deserialization, and validation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Discriminator {
    /// The name of the property in the payload that will hold the discriminator value.
    #[serde(rename = "propertyName")]
    pub property_name: String,
    /// An object to hold mappings between payload values and schema names or references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<BTreeMap<String, String>>,
}

/// ### XML
/// A metadata object that allows for more fine-tuned XML model definitions.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct XML {
    /// Replaces the name of the element/attribute used for the described schema property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URI of the namespace definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// The prefix to be used for the name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Declares whether the property definition translates to an attribute instead of an element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<bool>,
    /// MAY be used only for an array definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<bool>,
}

/// ### SecurityScheme
/// Defines a security scheme that can be used by the operations.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SecurityScheme {
    ///  The type of the security scheme.
    pub r#type: SecuritySchemeType,
    /// A short description for security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the header, query or cookie parameter to be used.
    pub name: String,
    /// The location of the API key. Valid values are "query", "header" or "cookie".
    pub r#in: String,
    /// The name of the HTTP Authorization scheme to be used in the [Authorization header as defined in RFC7235](https://tools.ietf.org/html/rfc7235#section-5.1).
    pub scheme: String,
    /// A hint to the client to identify how the bearer token is formatted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "beareFormat")]
    pub beare_format: Option<String>,
    ///  An object containing configuration information for the flow types supported.
    pub flows: OAuthFlows,
    ///  OpenId Connect URL to discover OAuth2 configuration values.
    #[serde(rename = "openIdConnectUrl")]
    pub open_id_connect_url: String,
}

/// #### SecuritySchemeType
/// for SecurityScheme.type
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SecuritySchemeType {
    ApiKey,
    Http,
    Oauth2,
    OpenIdConnect,
}

/// ### OAuthFlows
///
/// Allows configuration of the supported OAuth Flows.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OAuthFlows {
    /// Configuration for the OAuth Implicit flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlow>,
    /// Configuration for the OAuth Resource Owner Password flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlow>,
    /// Configuration for the OAuth Client Credentials flow. Previously called application in OpenAPI 2.0.
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientCredentials")]
    pub client_credentials: Option<OAuthFlow>,
    /// Configuration for the OAuth Authorization Code flow. Previously called accessCode in OpenAPI 2.0.
    #[serde(skip_serializing_if = "Option::is_none", rename = "authorizationCode")]
    pub authorization_code: Option<OAuthFlow>,
}

/// ### OAuthFlow
/// Configuration details for a supported OAuth Flow
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OAuthFlow {
    /// The authorization URL to be used for this flow.
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,
    /// The token URL to be used for this flow.
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens.
    #[serde(rename = "refreshUrl", skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme.
    pub scopes: BTreeMap<String, String>,
}

/// ### SecurityRequirement
/// Lists the required security schemes to execute this operation.
pub type SecurityRequirement = BTreeMap<String, Vec<String>>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum RefOrObject<T> {
    Ref(Reference),
    Object(T),
}
