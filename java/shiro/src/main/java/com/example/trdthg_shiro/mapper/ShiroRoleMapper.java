package com.example.trdthg_shiro.mapper;

import com.example.trdthg_shiro.entity.ShiroRole;
import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import org.apache.ibatis.annotations.Select;
import org.springframework.stereotype.Repository;

import java.util.List;

/**
 * <p>
 *  Mapper 接口
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
@Repository
public interface ShiroRoleMapper extends BaseMapper<ShiroRole> {
    @Select("SELECT r.function_name " +
            "FROM shiro_user u, shiro_role r, shiro_user_role ur " +
            "WHERE ur.user_id = #{id} AND r.id = ur.role_id")
    List<String> findByUserId(Integer id);
    //

}
