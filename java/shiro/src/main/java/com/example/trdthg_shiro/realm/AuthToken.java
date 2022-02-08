package com.example.trdthg_shiro.realm;


import org.apache.shiro.authc.UsernamePasswordToken;

public class AuthToken extends UsernamePasswordToken {

    private String token;
    private String username;

    @Override
    public String getUsername() {
        return username;
    }


    public AuthToken(String username, String token) {
        this.token = token;
        this.username = username;
    }

    @Override
    public Object getPrincipal() {
        return token;
    }

    @Override
    public Object getCredentials() {
        return token;
    }
}
