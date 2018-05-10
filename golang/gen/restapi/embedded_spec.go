// Code generated by go-swagger; DO NOT EDIT.

package restapi

// This file was generated by the swagger tool.
// Editing this file might prove futile when you re-run the swagger generate command

import (
	"encoding/json"
)

var (
	// SwaggerJSON embedded version of the swagger document used at generation time
	SwaggerJSON json.RawMessage
	// FlatSwaggerJSON embedded flattened version of the swagger document used at generation time
	FlatSwaggerJSON json.RawMessage
)

func init() {
	SwaggerJSON = json.RawMessage([]byte(`{
  "consumes": [
    "application/json"
  ],
  "produces": [
    "application/json"
  ],
  "schemes": [
    "http"
  ],
  "swagger": "2.0",
  "info": {
    "description": "A scalable, versioned, API-oriented catalog of bibliographic entities and file metadata",
    "title": "fatcat",
    "version": "0.1.0"
  },
  "host": "api.fatcat.wiki",
  "basePath": "/v0",
  "paths": {
    "/creator": {
      "post": {
        "parameters": [
          {
            "name": "body",
            "in": "body",
            "schema": {
              "$ref": "#/definitions/creator_entity"
            }
          }
        ],
        "responses": {
          "201": {
            "description": "created",
            "schema": {
              "$ref": "#/definitions/creator_entity"
            }
          },
          "default": {
            "description": "generic error response",
            "schema": {
              "$ref": "#/definitions/error"
            }
          }
        }
      }
    },
    "/creator/{id}": {
      "get": {
        "responses": {
          "200": {
            "description": "fetch a single creator by id",
            "schema": {
              "$ref": "#/definitions/creator_entity"
            }
          },
          "default": {
            "description": "generic error response",
            "schema": {
              "$ref": "#/definitions/error"
            }
          }
        }
      },
      "parameters": [
        {
          "type": "string",
          "name": "id",
          "in": "path",
          "required": true
        }
      ]
    }
  },
  "definitions": {
    "creator_entity": {
      "type": "object",
      "required": [
        "ident",
        "state"
      ],
      "properties": {
        "ident": {
          "type": "string"
        },
        "name": {
          "type": "string"
        },
        "orcid": {
          "type": "string"
        },
        "redirect": {
          "type": "string"
        },
        "revision": {
          "type": "string"
        },
        "state": {
          "type": "string"
        }
      }
    },
    "error": {
      "type": "object",
      "required": [
        "message"
      ],
      "properties": {
        "message": {
          "type": "string"
        }
      }
    }
  }
}`))
	FlatSwaggerJSON = json.RawMessage([]byte(`{
  "consumes": [
    "application/json"
  ],
  "produces": [
    "application/json"
  ],
  "schemes": [
    "http"
  ],
  "swagger": "2.0",
  "info": {
    "description": "A scalable, versioned, API-oriented catalog of bibliographic entities and file metadata",
    "title": "fatcat",
    "version": "0.1.0"
  },
  "host": "api.fatcat.wiki",
  "basePath": "/v0",
  "paths": {
    "/creator": {
      "post": {
        "parameters": [
          {
            "name": "body",
            "in": "body",
            "schema": {
              "$ref": "#/definitions/creator_entity"
            }
          }
        ],
        "responses": {
          "201": {
            "description": "created",
            "schema": {
              "$ref": "#/definitions/creator_entity"
            }
          },
          "default": {
            "description": "generic error response",
            "schema": {
              "$ref": "#/definitions/error"
            }
          }
        }
      }
    },
    "/creator/{id}": {
      "get": {
        "responses": {
          "200": {
            "description": "fetch a single creator by id",
            "schema": {
              "$ref": "#/definitions/creator_entity"
            }
          },
          "default": {
            "description": "generic error response",
            "schema": {
              "$ref": "#/definitions/error"
            }
          }
        }
      },
      "parameters": [
        {
          "type": "string",
          "name": "id",
          "in": "path",
          "required": true
        }
      ]
    }
  },
  "definitions": {
    "creator_entity": {
      "type": "object",
      "required": [
        "ident",
        "state"
      ],
      "properties": {
        "ident": {
          "type": "string"
        },
        "name": {
          "type": "string"
        },
        "orcid": {
          "type": "string"
        },
        "redirect": {
          "type": "string"
        },
        "revision": {
          "type": "string"
        },
        "state": {
          "type": "string"
        }
      }
    },
    "error": {
      "type": "object",
      "required": [
        "message"
      ],
      "properties": {
        "message": {
          "type": "string"
        }
      }
    }
  }
}`))
}