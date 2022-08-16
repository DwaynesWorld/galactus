# Galactus

Galactus is a cosmic entity who consumes planets to sustain his life force, and serves a functional role in the upkeep of the primary Marvel continuity.

## Endpoints

### Configuration
The endpoints create, update, delete and query configuration entries registered with Galactus

- List Configurations: `GET /configurations/:kind`
- Get Configuration: `GET /configurations/:kind/:name`
- Create Configuration:  `POST /configurations`
- Update Configuration:  `PUT /configurations/:kind/:name`
- Delete Configuration: `DELETE /configurations/:kind/:name`


### Subscriptions
The endpoints create, update, delete and query provide configuration for event subscriptions

- List Subscriptions: `GET /subscriptions`
- Get Subscription: `GET /subscriptions/:name`
- Create Subscription:  `POST /subscriptions`
- Update Subscription:  `PUT /subscriptions/:name`
- Delete Subscription: `DELETE /subscriptions/:name`