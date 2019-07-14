import React, { Component } from 'react';
import {Layout} from "../Layout";
import RootRoutes from "../RootRoutes";
import { BrowserRouter as Router } from 'react-router-dom';

export default class App extends Component {

    public render() {
        return (<>
          <Router>
            <Layout>
              <RootRoutes />
            </Layout>
          </Router>
        </>)
    }
}
