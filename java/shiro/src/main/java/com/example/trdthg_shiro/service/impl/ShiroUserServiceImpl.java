package com.example.trdthg_shiro.service.impl;

import com.alibaba.fastjson.JSONObject;
import com.auth0.jwt.JWT;
import com.auth0.jwt.JWTVerifier;
import com.auth0.jwt.algorithms.Algorithm;
import com.auth0.jwt.exceptions.JWTVerificationException;
import com.auth0.jwt.interfaces.DecodedJWT;
import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.example.trdthg_shiro.entity.ShiroUser;
import com.example.trdthg_shiro.mapper.ShiroUserMapper;
import com.example.trdthg_shiro.service.ShiroUserService;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.redis.core.HashOperations;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Service;

import java.util.Date;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeUnit;

/**
 * <p>
 * 服务实现类
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
@Service
public class ShiroUserServiceImpl extends ServiceImpl<ShiroUserMapper, ShiroUser> implements ShiroUserService {

    @Autowired
    ShiroUserMapper shiroUserMapper;

    public ShiroUser findByUsername(String username) {
        QueryWrapper<ShiroUser> wrapper = new QueryWrapper<>();
        wrapper.eq("username", username);
        ShiroUser shiroUser = shiroUserMapper.selectOne(wrapper);
        if (shiroUser != null) {
            return shiroUser;
        } else {
            return null;
        }
    }

    private static final String issuer = "trdthg";
    private static final long EXPIRE_TIME = 10;  // 10分钟
    private static final long EXPIRE_TIME_LONG = 60;  // 20分钟
    private static final String salt = "rae339o;d@!Q0+Q?q/2=5347%^$^%*&(*&#^";
    @Autowired
    RedisTemplate redisTemplate;

    public Map<String, String> saveToken(String username) {
        System.out.println("用户登录，保存两个token");
        Map<String, String> resultMap = new HashMap<>();
        String newAccessToken = createAccessToken(username);
        String newRefreshToken = createRefreshToken(username);
        resultMap.put("access_token", newAccessToken);
        resultMap.put("refresh_token", newRefreshToken);
        redisTemplate.opsForValue().set("access_token" + username, newAccessToken, EXPIRE_TIME, TimeUnit.SECONDS);
        redisTemplate.opsForValue().set("refresh_token" + username, newRefreshToken, EXPIRE_TIME_LONG, TimeUnit.SECONDS);
        return resultMap;
    }

    public Map<String, Object> refreshToken(String refreshToken) {
        System.out.println("根据refreshToken刷新过期的AccessToken，返回新的AccessToken");
        Map<String, Object> resultMap = new HashMap<>();
        System.out.println(refreshToken);

        // 判断refreshToken是否过期，是否存在
        Map<String, Object> tokenInfo = getInfoFromToken(refreshToken);
        String username = (String) tokenInfo.get("username");
        System.out.println(username);
        String temp = (String) redisTemplate.opsForValue().get("refresh_token" + username);
        System.out.println(temp);
        if (temp != null && temp.equals(refreshToken)) {
            Date expireAt = (Date) tokenInfo.get("expireAt");
            String newAccessToken = createAccessToken(username);
            resultMap.put("accessToken", newAccessToken);
            redisTemplate.opsForValue().set("access_token" + username, newAccessToken, EXPIRE_TIME, TimeUnit.SECONDS);

            // 判断refreshToken在用户活跃期间是否有可能过期，如果快了，就连同refreshToken也进行刷新
            if ((expireAt.getTime() - new Date().getTime()) < EXPIRE_TIME * 2) {
                System.out.println("refreshToken也快过期了");
                String newRefreshToken = createRefreshToken(username);
                resultMap.put("refreshToken", newRefreshToken);
                redisTemplate.opsForValue().set("refresh_token" + username, newRefreshToken, EXPIRE_TIME_LONG, TimeUnit.SECONDS);
            }
            return resultMap;
        }
        resultMap.put("status", 403);
        resultMap.put("msg", "refreshToken已过期，请重新登录");
        return resultMap;
    }

    public String createAccessToken(String username) {
        JSONObject header = JSONObject.parseObject("{\"alg\": \"HS256\", \"typ\": \"JWT\"}");
        return JWT.create()
                .withHeader(header)
                .withClaim("username", username)
                .withIssuer(issuer)
                .withExpiresAt(new Date(System.currentTimeMillis() + EXPIRE_TIME))
                .withIssuedAt(new Date())
                .sign(Algorithm.HMAC256(salt));
    }

    public String createRefreshToken(String username) {
        JSONObject header = JSONObject.parseObject("{\"alg\": \"HS256\", \"typ\": \"JWT\"}");
        return JWT.create()
                .withClaim("sss","sadw")
                .withHeader(header)
                .withClaim("username", username)
                .withIssuer(issuer)
                .withExpiresAt(new Date(System.currentTimeMillis() + EXPIRE_TIME_LONG))
                .withIssuedAt(new Date())
                .sign(Algorithm.HMAC256(salt));
    }

    public Map<String, Object> getInfoFromToken(String refreshToken) {
        DecodedJWT jwt = JWT.decode(refreshToken);
        Map<String, Object> resultMap = new HashMap<>();
        resultMap.put("username", jwt.getClaim("username").asString());
        resultMap.put("expireAt", jwt.getExpiresAt());
        return resultMap;
    }

//    public boolean verifyToken(String token){
//        JWT.require(Algorithm.HMAC256(salt)).build().verify(token);  // 如果验证通过，则不会把报错，否则会报错
//        try {
//            Algorithm algorithm = Algorithm.HMAC256("secret");
//            JWTVerifier verifier = JWT.require(algorithm)
//                    .withIssuer(issuer) //匹配指定的token发布者 auth0
//                    .build();
//            DecodedJWT jwt = verifier.verify(token); //解码JWT ，verifier 可复用
//            return true;
//        }catch (JWTVerificationException e){
//            //无效的签名/声明
//            e.printStackTrace();
//        }
//        return false;
//    }

}
