import React, {Component} from "react";
import {Grid, Paper} from "@material-ui/core";
import styles from './AggregateList.module.scss';


export default class AggregateList extends Component {

  public render() {
    return (
      <Grid container spacing={3}>
        <Grid item xs={12} md={6} lg={4}>
          <Paper className={styles.aggregatePaper}>
            Aggregate
          </Paper>
        </Grid>
      </Grid>
    );
  }
}
