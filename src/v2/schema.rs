use crate::extension::Extensions;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::collections::BTreeMap;

/// # Swagger
///
/// The Top Level Struct of swagger
///
/// Swagger document specification [here](https://swagger.io/specification/v2/)
///
/// see https://swagger.io/specification/v2/#swagger-object
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Swagger {
    /// 2.0
    pub swagger: String,
    /// Provides metadata about the API. The metadata can be used by the clients if needed.
    pub info: Info,
    /// The host (name or ip) serving the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// The base path on which the API is served, which is relative to the host.
    #[serde(skip_serializing_if = "Option::is_none", rename = "basePath")]
    pub base_path: Option<String>,
    /// The transfer protocol of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<TransferProtocol>>,
    /// A list of MIME types the APIs can consume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumes: Option<String>,
    /// A list of MIME types the APIs can produce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produces: Option<String>,

    /// The available paths and operations for the API.
    pub paths: BTreeMap<String, PathItem>,

    /// An object to hold data types produced and consumed by operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<BTreeMap<String, Schema>>,

    /// An object to hold parameters that can be used across operations. This property does not define global parameters for all operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, Parameter>>,

    /// An object to hold responses that can be used across operations. This property does not define global responses for all operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<BTreeMap<String, Response>>,

    /// Security scheme definitions that can be used across the specification.
    #[serde(skip_serializing_if = "Option::is_none", rename = "SecurityScheme")]
    pub security_definitions: Option<BTreeMap<String, SecurityScheme>>,

    /// A declaration of which security schemes are applied for the API as a whole.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,

    /// A list of tags used by the specification with additional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<ExternalDoc>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with `x-`, for example, `x-internal-id`. The value can be null, a primitive, an array or an object.
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ## Info
///
/// The object provides metadata about the API. The metadata can be used by the clients if needed, and can be presented in the Swagger-UI for convenience.
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
///
/// Contact information for the exposed API.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information. MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The email address of the contact person/organization. MUST be in the format of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### License
///
/// License information for the exposed API.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct License {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API. MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### TransferProtocol
///
/// A list of protocol
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum TransferProtocol {
    #[default]
    Http,
    Https,
    Ws,
    Wss,
}

/// ### ExternalDoc
///
/// Allows referencing an external resource for extended documentation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ExternalDoc {
    /// A short description of the target documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL for the target documentation.
    pub url: String,
}

/// ### Tag
///
/// Allows adding meta data to a single tag that is used by the Operation Object.
///
/// It is not mandatory to have a Tag Object per tag used there.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Tag {
    /// The name of the tag
    pub name: String,
    /// A short description for the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDoc")]
    pub external_doc: Option<ExternalDoc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InEnum {
    Query,
    Header,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Flow {
    Implicit,
    Password,
    Application,
    AccessCode,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Scopes {
    Implicit,
    Password,
    Application,
    AccessCode,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuthorizationUrl {
    Implicit,
    AccessCode,
}
/// ### SecuritySchemeType
///
/// The type of the security scheme. Valid values are "basic", "apiKey" or "oauth2".
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SecuritySchemeType {
    Basic,
    ApiKey,
    Oauth2,
}

/// ### TokenUrl
///
///  The token URL to be used for this flow.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TokenUrl {
    Password,
    Application,
    AccessCode,
}
/// ## SecurityScheme
///
/// A declaration of the security schemes available to be used in the specification.
///
/// This does not enforce the security schemes on the operations and only serves to provide the relevant details for each scheme.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SecurityScheme {
    /// The type of the security scheme. Valid values are `"basic"`, `"apiKey"` or `"oauth2"`.
    pub r#type: SecuritySchemeType,
    /// A short description for security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the header or query parameter to be used.
    pub name: String,
    /// The location of the API key. Valid values are `"query"` or `"header"`.
    pub r#in: InEnum,
    /// The flow used by the OAuth2 security scheme. Valid values are `"implicit"`,`"password"`,`"application"` or `"accessCode"`.
    pub flow: Flow,
    /// The authorization URL to be used for this flow. This SHOULD be in the form of a URL.
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: AuthorizationUrl,
    /// The token URL to be used for this flow. This SHOULD be in the form of a URL.
    #[serde(rename = "tokenUrl")]
    pub token_url: TokenUrl,
    /// The available scopes for the OAuth2 security scheme.
    pub scopes: BTreeMap<String, String>,
    /// Allows extensions to the Swagger Schema.
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### SecurityRequirementObject
///
/// Lists the required security schemes to execute this operation.
/// The object can have multiple security schemes declared in it which are all required (that is, there is a logical AND between the schemes).
///
/// The name used for each property MUST correspond to a security scheme declared in the Security Definitions.
/// see https://swagger.io/specification/v2/#securityRequirementObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SecurityRequirementObject(BTreeMap<String, Vec<String>>);

/// ### Path Item
/// Describes the operations available on a single path.
/// A Path Item may be empty, due to [ACL constraints](https://swagger.io/specification/v2/#securityFiltering).
/// The path itself is still exposed to the documentation viewer but they will not know which operations and parameters are available.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PathItem {
    /// Allows for an external definition of this path item. The referenced structure MUST be in the format of a Path Item Object.
    /// If there are conflicts between the referenced definition and this Path Item's definition, the behavior is undefined.
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
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
    /// A list of parameters that are applicable for all the operations described under this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterOrRef>>,
}

/// ### Operation
/// Describes a single API operation on a path.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Operation {
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of operations by resources or any other qualifier.
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
    /// A list of MIME types the operation can consume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumes: Option<Vec<String>>,
    /// A list of MIME types the operation can produce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produces: Option<Vec<String>>,
    /// A list of parameters that are applicable for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterOrRef>>,
    /// The list of possible responses as they are returned from executing this operation.
    pub responses: Responses,
    /// The transfer protocol for the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<TransferProtocol>,
    /// Declares this operation to be deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<String>,
    /// A declaration of which security schemes are applied for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ParamInEnum {
    Query,
    Header,
    Path,
    FormData,
    Body,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ParameterOrRef {
    Ref(Reference),
    Parameter(Parameter),
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Reference {
    #[serde(rename = "$ref")]
    reference: String,
}

/// ### Parameter
///
/// Describes a single operation parameter. see https://swagger.io/specification/v2/#parameter-object.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// The location of the parameter. Possible values are "query", "header", "path", "formData" or "body".
    pub r#in: ParamInEnum,
    /// A brief description of the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory.
    /// If the parameter is in "path", this property is **required** and its value MUST be `true`. Otherwise, the property MAY be included and its default value is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// The schema defining the type used for the body parameter.
    /// If the parameter is in "body", this property is **required**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// The type of the parameter. Since the parameter is not located at the request body, it is limited to simple types (that is, not an object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ParameterType>,
    /// The extending format for the previously mentioned type. See [Data Type Formats](https://swagger.io/specification/v2/#dataTypeFormat) for further details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Sets the ability to pass empty-valued parameters.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>,

    /// **Required if `type` is "array"**. Describes the type of items in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Items>,
    /// Determines the format of the array if type array is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "collectionFormat")]
    pub collection_format: Option<String>,

    /// Declares the value of the parameter that the server will use if none is provided, for example a "count" to control the number of results per page might default to 100 if not supplied by the client in the request.
    /// (Note: "default" has no meaning for required parameters.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
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
    pub r#enum: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multipleOf")]
    pub multiple_of: Option<Number>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### Schema Object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// [JSON reference](https://tools.ietf.org/html/draft-pbryan-zyp-json-ref-03)
    /// path to another defintion
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub r#enum: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "multipleOf")]
    pub multiple_of: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    // implies object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, Schema>>,
    // composition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<Box<Schema>>>,
    // TODO: we need a validation step that we only collect x-* properties here.
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

/// ### ParameterType
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    File,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemsType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
}
/// ### Items Object
/// A limited subset of JSON-Schema's items object. It is used by parameter definitions that are not located in "body".
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Items {
    /// The internal type of the array. The value MUST be one of "string", "number", "integer", "boolean", or "array". Files and models are not allowed.
    pub r#type: ItemsType,

    /// The extending format for the previously mentioned `type`. See [Data Type Formats](https://swagger.io/specification/v2/#dataTypeFormat) for further details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// **Required if `type` is "array"**. Describes the type of items in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Items>>,

    /// Determines the format of the array if type array is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "collectionFormat")]
    pub collection_format: Option<String>,

    /// Declares the value of the parameter that the server will use if none is provided, for example a "count" to control the number of results per page might default to 100 if not supplied by the client in the request.
    /// (Note: "default" has no meaning for required parameters.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
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
    pub r#enum: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multipleOf")]
    pub multiple_of: Option<Number>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### Responses
/// A container for the expected responses of an operation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Responses(BTreeMap<String, ResponseOrRef>);

/// ### Response
/// Describes a single response from an API Operation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Response {
    ///  A short description of the response.
    pub description: String,
    /// A definition of the response structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// A list of headers that are sent with the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, Header>>,
    /// An example of the response message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<BTreeMap<String, Value>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// ### Header
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Header {
    /// A short description of the header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The internal type of the array. The value MUST be one of "string", "number", "integer", "boolean", or "array". Files and models are not allowed.
    pub r#type: ItemsType,

    /// The extending format for the previously mentioned `type`. See [Data Type Formats](https://swagger.io/specification/v2/#dataTypeFormat) for further details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// **Required if `type` is "array"**. Describes the type of items in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Items>>,

    /// Determines the format of the array if type array is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "collectionFormat")]
    pub collection_format: Option<String>,

    /// Declares the value of the parameter that the server will use if none is provided, for example a "count" to control the number of results per page might default to 100 if not supplied by the client in the request.
    /// (Note: "default" has no meaning for required parameters.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxLength")]
    pub max_length: Option<Value>,
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
    pub r#enum: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multipleOf")]
    pub multiple_of: Option<Number>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseOrRef {
    Response {
        description: String,
        schema: Option<Schema>,
        headers: Option<BTreeMap<String, Header>>,
        examples: Option<BTreeMap<String, Value>>,
        #[serde(flatten)]
        extensions: Extensions,
    },
    Ref {
        #[serde(rename = "$ref")]
        reference: String,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    #[test]
    fn parameter_or_ref_dese() {
        let json_str = r#"{
            "$ref":"/some/path"
        }"#;

        assert_eq!(
            serde_json::from_str::<ParameterOrRef>(&json_str).unwrap(),
            ParameterOrRef::Ref(Reference {
                reference: "/some/path".to_string()
            })
        )
    }
}
