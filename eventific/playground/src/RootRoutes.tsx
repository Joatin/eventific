import React, { Component } from 'react';
import { Redirect, Route, Switch } from 'react-router';
import { NotFound } from './NotFound';
import {AggregateList} from "./AggregateList";

export default class RootRoutes extends Component {
  public render() {
    return (
      <Switch>
        <Redirect from="/" exact to="/discover" />
        <Route path="/discover" exact component={AggregateList} />
        <Route path="**" component={NotFound} />
      </Switch>
    );
  }
}
