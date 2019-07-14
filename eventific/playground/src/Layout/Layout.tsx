import React, {Component} from "react";
import {
  AppBar,
  IconButton,
  Toolbar,
  Typography,
  SwipeableDrawer,
  List,
  ListItem,
  ListItemIcon, ListItemText, Container
} from "@material-ui/core";
import MenuIcon from '@material-ui/icons/Menu';
import Explore from '@material-ui/icons/Explore';
import styles from './Layout.module.scss'
import {Link} from "react-router-dom";

export default class Layout extends Component {
    state = {
      drawerOpen: false
    };

    public render() {
      const { children } = this.props;

        return (
            <>
              <AppBar>
                <Toolbar>
                  <IconButton
                    edge="start"
                    color="inherit"
                    aria-label="Open drawer"
                    onClick={this.handleDrawerOpen}
                  >
                    <MenuIcon />
                  </IconButton>
                  <Typography component="h1" variant="h6" color="inherit" noWrap>
                    Eventific
                  </Typography>
                </Toolbar>
              </AppBar>
              <SwipeableDrawer
                open={this.state.drawerOpen}
                onOpen={this.handleDrawerOpen}
                onClose={this.handleDrawerClose}
              >
                <div className={styles.list} role="presentation">
                  <List>
                    <ListItem button >
                      <ListItemIcon><Explore /></ListItemIcon>
                      <ListItemText primary={"Discover"} />
                    </ListItem>
                  </List>
                </div>
              </SwipeableDrawer>
              <main className={styles.main}>
                <Container maxWidth="lg">
                  { children }
                </Container>
              </main>
            </>
        );
    }

  private handleDrawerOpen = () => {
    this.setState({
      drawerOpen: true
    })
  };

  private handleDrawerClose = () => {
    this.setState({
      drawerOpen: false
    })
  };
}
