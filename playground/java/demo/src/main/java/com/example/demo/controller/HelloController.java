package com.example.demo.controller;

import java.io.Serializable;

import com.fasterxml.jackson.core.sym.Name;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.ResponseBody;

import io.swagger.annotations.Api;
import io.swagger.annotations.ApiOperation;
import io.swagger.annotations.ApiParam;
import io.swagger.annotations.*;

@Controller
@Api(value="用户controller",tags={"用户操作接口"})
public class HelloController {

    @ApiOperation("更改用户信息")
    @PostMapping("/updateUserInfo")
    public int updateUserInfo(@RequestBody @ApiParam(name="用户对象",value="传入json格式",required=true) User user){

    //    int num = userService.updateUserInfo(user);
       return 1;
    }

    @ApiOperation(value = "你好")
    @ResponseBody
    @PostMapping("/hello/{name}")
    public User hello(@PathVariable(name = "name") String name,
            @RequestBody @ApiParam(name = "用户对象", value = "传入json格式", required = true) User user) {
        return new User();
    }

    @ApiOperation(value = "你好2")
    @ResponseBody
    @PostMapping("/hello2")
    public String hello2(@ApiParam(name = "name", value = "对话人", required = true) String name) {
        return name + ", hello";
    }

    @ApiOperation(value = "你好3")
    @ResponseBody
    @PostMapping("/hello3")
    public String hello3(@ApiParam(name = "name", value = "对话人", required = true) String name) {
        return name + ", hello";
    }
}

@ApiModel()
class User implements Serializable {
    private static final long serialVersionUID = 1L;
    @ApiModelProperty(value = "用户名", name = "username", example = "xingguo")
    private String username;
    @ApiModelProperty(value = "状态", name = "state", required = true, example = "1 / 0", dataType = "java.lang.Byte")
    private Integer state;
    private String password;
    private String nickName;
    private Integer isDeleted;
    @ApiModelProperty()
    private String id;
    private Double length;
    private Byte status;

    public Double getLength() {
        return length;
    }
    public void setLength(Double length) {
        this.length = length;
    }
    public Byte getStatus() {
        return status;
    }
    public void setStatus(Byte status) {
        this.status = status;
    }
    @ApiModelProperty(value = "id数组", hidden = true)
    private String[] ids;
    public static long getSerialversionuid() {
        return serialVersionUID;
    }
    public String getUsername() {
        return username;
    }
    public void setUsername(String username) {
        this.username = username;
    }
    public Integer getState() {
        return state;
    }
    public void setState(Integer state) {
        this.state = state;
    }
    public String getPassword() {
        return password;
    }
    public void setPassword(String password) {
        this.password = password;
    }
    public String getNickName() {
        return nickName;
    }
    public void setNickName(String nickName) {
        this.nickName = nickName;
    }
    public Integer getIsDeleted() {
        return isDeleted;
    }
    public void setIsDeleted(Integer isDeleted) {
        this.isDeleted = isDeleted;
    }
    public String[] getIds() {
        return ids;
    }
    public void setIds(String[] ids) {
        this.ids = ids;
    }
    public java.util.List<String> getIdList() {
        return idList;
    }
    public void setIdList(java.util.List<String> idList) {
        this.idList = idList;
    }
    private java.util.List<String> idList;
    // 省略get/set
}