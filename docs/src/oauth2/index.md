# OAuth2
Wilford (partially) implements RFC6749.

## Terminology
This document follows the terminology used in [RFC6749 Section 1.1](https://datatracker.ietf.org/doc/html/rfc6749#section-1.1):


#### Resource owner
An entity capable of granting access to a protected resource.
When the resource owner is a person, it is referred to as an
end-user.

#### Resource server
The server hosting the protected resources, capable of accepting
and responding to protected resource requests using access tokens.

#### Client 
An application making protected resource requests on behalf of the
resource owner and with its authorization.  The term "client" does
not imply any particular implementation characteristics (e.g.,
whether the application executes on a server, a desktop, or other
devices).

This is your application.

#### Authorization server
The server issuing access tokens to the client after successfully
authenticating the resource owner and obtaining authorization.

`Wilford` fulfils this role